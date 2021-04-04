use rayon::prelude::*;
use tsplib::Tsp;
use crate::matrix::SymmetricMatrix;
use crate::path::Path;

pub mod matrix;
pub mod path;
pub mod route;

fn local_search_step(tsp: &SymmetricMatrix, candidate: &mut Path) -> Option<((usize, usize), (usize, usize))> {
    candidate.edges_visited()
        .collect::<Vec<_>>() // TODO: Implement IndexedParallelIterator to avoid having to collect
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

pub fn local_search(tsp: &SymmetricMatrix, candidate: &mut Path) {
    while let Some((v, c)) = local_search_step(tsp, candidate) {
        candidate.twist(v, c)
    }
}

pub fn load_problem() -> SymmetricMatrix {
    let home = env!("CARGO_MANIFEST_DIR").to_owned();
    let tsp = Tsp::from_file(&(home + "/data/pcb3038.tsp")).unwrap();
    SymmetricMatrix::from_tsplib(&tsp)
}

#[allow(dead_code)]
fn main() {
    let tsp = load_problem();
    let candidate_route = tsp.nearest_neighbor();
    let mut candidate = candidate_route.path;

    println!("{:.25}", tsp);
    println!("{:?}", candidate.vertices_visited().collect::<Vec<_>>());
    local_search(&tsp, &mut candidate);
    println!("{:?}", candidate.vertices_visited().collect::<Vec<_>>());
    println!("{}", tsp.cost(&candidate));
}

#[cfg(test)]
mod tests {
    use crate::main;

    #[test]
    fn test() {
        main()
    }
}
