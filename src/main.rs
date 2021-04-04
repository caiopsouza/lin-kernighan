use tsplib::Tsp;
use crate::matrix::SymmetricMatrix;

mod matrix;
mod path;
mod route;

fn linker(tsp: &SymmetricMatrix) {
    let candidate_route = tsp.nearest_neighbor();
    let mut candidate = candidate_route.path;

    println!("{:.25}", tsp);
    println!("{:?}", candidate.vertices_visited().collect::<Vec<_>>());

    let mut edges_visited = candidate.edges_visited();
    let (v0, v1) = edges_visited.next().unwrap();
    let initial_cost = tsp[(v0, v1)];

    for (c0, c1) in edges_visited.skip(1) {
        let cost_decrease = initial_cost + tsp[(c0, c1)];
        let cost_increase = tsp[(v0, c1)] + tsp[(c0, v1)];

        if cost_decrease > cost_increase {
            candidate.twist((v0, v1), (c0, c1));
            break;
        }
    }
    println!("{:?}", candidate.vertices_visited().collect::<Vec<_>>());
}

fn main() {
    let home = env!("CARGO_MANIFEST_DIR").to_owned();
    let tsp = Tsp::from_file(&(home + "/data/pcb3038.tsp")).unwrap();
    let tsp = SymmetricMatrix::from_tsplib(&tsp);
    linker(&tsp);
}
