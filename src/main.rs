use rayon::prelude::*;
use tsplib::Tsp;
use crate::matrix::SymmetricMatrix;
use crate::path::Path;
use crate::route::Route;

pub mod matrix;
pub mod path;
pub mod route;

fn local_search_step(tsp: &SymmetricMatrix, candidate: &mut Path, edge_buffer: &mut Vec<(usize, usize)>) -> Option<((usize, usize), (usize, usize))> {
    // TODO: Implement IndexedParallelIterator to avoid having to collect. `par_bridge` has worse performance.
    edge_buffer.clear();
    edge_buffer.extend(candidate.edges_visited());

    edge_buffer
        .par_iter()
        .find_map_any(|&(v0, v1)| {
            let initial_cost = tsp[(v0, v1)];

            for (c0, c1) in candidate.edges_visited_after(v0, v1) {
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

    let edge_buffer = &mut vec![(0usize, 0usize); tsp.size()];
    let mut tsp_with_penalties = tsp.clone();

    local_search(&tsp, &mut route, edge_buffer);
    route.cost = tsp.cost(&route.path);

    let mut penalties = SymmetricMatrix::from_size(size);
    let penalty_factor = (0.3 * (route.cost as f64 / size as f64)) as u32;

    for _ in 0..steps {
        let calc_utility = |e: (usize, usize)| -> i32 {
            (tsp[e] as f64 / (1.0 + penalties[e] as f64)) as i32
        };

        // Find the maximum utility
        let max_utility = route.path.edges_visited()
            .map(calc_utility)
            .max()
            .unwrap();

        route.path.edges_visited()
            .filter(|&e| calc_utility(e) == max_utility)
            .collect::<Vec<_>>()
            .iter().for_each(|&(e0, e1)|
            {
                let penalty = penalties.inc(e0, e1, 1);
                tsp_with_penalties.set(e0, e1, tsp[(e0, e1)] + penalty_factor * penalty);
            });

        local_search(&tsp_with_penalties, &mut route, edge_buffer);
    }

    // Guarantee it's at least on a local minimum
    local_search(&tsp, &mut route, edge_buffer);
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
