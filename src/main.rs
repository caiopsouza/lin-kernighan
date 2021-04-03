use tsplib::Tsp;
use crate::matrix::SymmetricMatrix;

mod matrix;
mod path;
mod route;

fn linker(tsp: &SymmetricMatrix) {
    let candidate = tsp.nearest_neighbor();
    println!("{}", candidate);
    println!("{:?}", candidate.path.vertices_visited());
}

fn main() {
    let home = env!("CARGO_MANIFEST_DIR").to_owned();
    let tsp = Tsp::from_file(&(home + "/data/pcb3038.tsp")).unwrap();
    let tsp = SymmetricMatrix::from_tsplib(&tsp);
    linker(&tsp);
}
