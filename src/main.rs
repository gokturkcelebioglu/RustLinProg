use solver::Solver;

pub mod linear_algebra;
pub mod solver;

fn main() {

    let mut model = Solver::read_json("./model.json".to_string());
    Solver::optimize(&mut model);

}
