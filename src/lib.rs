use tsplib::Tsp;
use crate::matrix::SymmetricMatrix;
use crate::path::Path;

pub mod matrix;
pub mod path;
pub mod route;

pub fn local_search_step(tsp: &SymmetricMatrix, candidate: &mut Path) -> bool {
    let mut edges = candidate.edges_visited();
    while let Some((v0, v1)) = edges.next() {
        let initial_cost = tsp[(v0, v1)];

        let mut neighbors = edges.clone().skip(1);
        while let Some((c0, c1)) = neighbors.next() {
            let cost_decrease = initial_cost + tsp[(c0, c1)];
            let cost_increase = tsp[(v0, c0)] + tsp[(v1, c1)];

            if cost_decrease > cost_increase {
                candidate.twist((v0, v1), (c0, c1));
                return true;
            }
        }
    }
    false
}

pub fn load_problem() -> SymmetricMatrix {
    let home = env!("CARGO_MANIFEST_DIR").to_owned();
    let tsp = Tsp::from_file(&(home + "/data/pcb3038.tsp")).unwrap();
    SymmetricMatrix::from_tsplib(&tsp)
}

fn main() {
    let tsp = load_problem();
    let candidate_route = tsp.nearest_neighbor();
    let mut candidate = candidate_route.path;

    println!("{:.25}", tsp);
    println!("{:?}", candidate.vertices_visited().collect::<Vec<_>>());
    while local_search_step(&tsp, &mut candidate) {}
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
