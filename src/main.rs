use rayon::prelude::*;
use tsplib::Tsp;
use crate::matrix::SymmetricMatrix;
use crate::path::Path;
use crate::route::Route;
use std::borrow::Borrow;

pub mod matrix;
pub mod path;
pub mod route;

fn local_search_step(tsp: &SymmetricMatrix, candidate: &mut Path, edge_buffer: &mut Vec<(usize, usize)>) -> Option<((usize, usize), (usize, usize))> {
    // TODO: Implement IndexedParallelIterator to avoid having to collect. `par_bridge` has worse performance.
    candidate.edges_visited_buffered(edge_buffer);

    edge_buffer
        .par_iter()
        .copied()
        .enumerate()
        .find_map_any(|(i, (v0, v1))| {
            let initial_cost = tsp[(v0, v1)];

            let neighbors = (edge_buffer.borrow() as &Vec<(usize, usize)>)
                .into_iter()
                .copied()
                .skip(i + 2);

            for (c0, c1) in neighbors {
                let cost_decrease = initial_cost + tsp[(c0, c1)];
                let cost_increase = tsp[(v0, c0)] + tsp[(v1, c1)];

                if cost_decrease > cost_increase {
                    return Some(((v0, v1), (c0, c1)));
                }
            }

            None
        })
}

pub fn local_search(tsp: &SymmetricMatrix, candidate: &mut Route, edge_buffer: &mut Vec<(usize, usize)>) {
    while let Some((v, c)) = local_search_step(tsp, &mut candidate.path, edge_buffer) {
        candidate.path.twist(v, c)
    }
}

pub fn load_problem() -> SymmetricMatrix {
    let home = env!("CARGO_MANIFEST_DIR").to_owned();
    let tsp = Tsp::from_file(&(home + "/data/pcb3038.tsp")).unwrap();
    SymmetricMatrix::from_tsplib(&tsp)
}

pub fn gls(tsp: &SymmetricMatrix, steps: usize) -> Route {
    let size = tsp.size();
    let mut route = tsp.nearest_neighbor();

    println!("{:.25}", tsp);
    println!("{:?}", route.path.vertices_visited().collect::<Vec<_>>());

    let mut edge_buffer = vec![(0usize, 0usize); tsp.size()];
    let mut tsp_with_penalties = tsp.clone();

    local_search(&tsp, &mut route, &mut edge_buffer);
    route.cost = tsp.cost(&route.path);

    let mut penalties = SymmetricMatrix::from_size(size);
    let penalty_factor = (0.3 * (route.cost as f64 / size as f64)) as u32;

    for _ in 0..steps {
        let calc_utility = |penalties: &SymmetricMatrix, e: (usize, usize)| -> i32 {
            (tsp[e] as f64 / (1.0 + penalties[e] as f64)) as i32
        };

        // Find the maximum utility
        // The edge buffer will have the correct edges because the last iteration of the local search doesn't change edges.
        let max_utility = edge_buffer
            .iter()
            .copied()
            .map(|e| calc_utility(&penalties, e))
            .max()
            .unwrap();

        for &edge in edge_buffer.iter() {
            if calc_utility(&penalties, edge) == max_utility {
                let penalty = penalties.inc(edge, 1);
                tsp_with_penalties.set(edge, tsp[edge] + penalty_factor * penalty);
            }
        }

        local_search(&tsp_with_penalties, &mut route, &mut edge_buffer);
    }

    // Guarantee it's at least on a local minimum
    local_search(&tsp, &mut route, &mut edge_buffer);
    route.cost = tsp.cost(&route.path);

    println!("{}", route.cost);
    println!("{:?}", route.path.vertices_visited().collect::<Vec<_>>());

    route
}

#[allow(dead_code)]
fn main() {
    let tsp = load_problem();
    gls(&tsp, 10000);
}
