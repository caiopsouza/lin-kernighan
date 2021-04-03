use tsplib::Tsp;
use crate::matrix::SymmetricMatrix;

mod matrix;
mod path;
mod route;

fn main() {
    let home = env!("CARGO_MANIFEST_DIR").to_owned();
    let tsp = Tsp::from_file(&(home + "/data/pcb3038.tsp")).unwrap();
    let tsp = SymmetricMatrix::from_tsplib(&tsp);

    println!("{:.25}", tsp);
}
