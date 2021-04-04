use tsplib::Tsp;
use crate::matrix::SymmetricMatrix;

mod matrix;
mod path;
mod route;

fn linker(tsp: &SymmetricMatrix) {
    let candidate_route = tsp.nearest_neighbor();
    let candidate = candidate_route.path;

    println!("{:?}", candidate);
    println!("{}", candidate);
    println!("{:?}", candidate.edges_visited().collect::<Vec<_>>());

    let mut edges_visited = candidate.edges_visited();
    let edge_cost = tsp[edges_visited.next().unwrap()];

    for edge in edges_visited.skip(1) {
        if tsp[edge] > edge_cost {
            panic!("{:?}, {:?}, {}, {}\n", candidate.edges_visited().next().unwrap(), edge, tsp[edge], edge_cost);
        }
    }
}

fn main() {
    let home = env!("CARGO_MANIFEST_DIR").to_owned();
    let tsp = Tsp::from_file(&(home + "/data/pcb3038.tsp")).unwrap();
    let tsp = SymmetricMatrix::from_tsplib(&tsp);
    linker(&tsp);
}
