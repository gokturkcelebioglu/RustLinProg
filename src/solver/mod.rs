use std::{fmt, fs};

use crate::linear_algebra::functions::matrix_product;

use self::structs::Model;
mod model;
pub mod structs;
mod tests;

#[derive(Default, Debug)]
pub(crate) struct Tableau {
    pub(crate) matrix: Vec<Vec<f64>>,
    pub(crate) row_names: Vec<String>,
    pub(crate) column_names: Vec<String>,
}

#[derive(Default)]
pub struct SolverResult {
    pub(crate) results: Vec<(String, f64)>,
}

#[allow(unused_must_use)]
impl fmt::Debug for SolverResult {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        println!("\n-------------------------");
        println!("RESULT:");
        println!("-------------------------");
        println!("Variable  Value");

        for variable in &self.results {
            let variable_name = &variable.0;
            let value = format!("{:.2}", variable.1);
            println!("{:9} {}", variable_name, value);
        }
        println!("-------------------------");

        Ok(())
    }
}

pub struct Solver;

impl Solver {
    /// Reads given json file to create model. Returns model.
    pub fn read_json(path: String) -> Model {
        let file: String =
            fs::read_to_string(path).expect("Should have been able to read the file");
        let model: Model = serde_json::from_str(&file).unwrap();
        model
    }

    /// Prepares tableau from the model, then recalculates the tableau until the optimality condition is reached.
    pub fn optimize(model: &mut Model) {
        let mut tableau: Tableau = Default::default();
        tableau.row_names = model.add_slack_variables();
        tableau.column_names = model.get_all_variable_names();
        let matrix = model.create_matrix(tableau.column_names.clone());
        tableau.matrix = matrix;

        Solver::write_tableau_to_console(&tableau);

        // Recalculate model until the optimality condition is reached.
        while !Solver::is_optimality_condition_reached(&tableau) {
            let entering_variable_index = Solver::get_entering_variable_index(&mut tableau);
            let leaving_variable_index =
                Solver::get_leaving_variable_index(&mut tableau, entering_variable_index);

            Solver::recalculate_model(
                &mut tableau,
                leaving_variable_index,
                entering_variable_index,
            );
            tableau.row_names[leaving_variable_index] =
                tableau.column_names[entering_variable_index].clone();
            Solver::write_tableau_to_console(&tableau);
        }

        let mut solver_result: SolverResult = Default::default();

        for row_index in 0..tableau.matrix.len() {
            let variable_name = &tableau.row_names[row_index];
            let result = &tableau.matrix[row_index][tableau.matrix[row_index].len() - 1];
            solver_result
                .results
                .push((variable_name.to_string(), *result));
        }

        println!("{:?}", solver_result);
    }

    /// Looks first row of the model for getting entering variable index.
    pub(self) fn get_entering_variable_index(tableau: &mut Tableau) -> usize {
        let mut entering_variable_index: usize = 0;

        let objective = tableau.matrix[0].clone();

        let number_of_columns = tableau.column_names.len();

        // Last item is right side constant.
        for index in 1..number_of_columns - 1 {
            if objective[entering_variable_index] > objective[index] {
                entering_variable_index = index;
            }
        }
        entering_variable_index
    }

    /// Looks constraint for getting entering variable index.
    pub(self) fn get_leaving_variable_index(
        tableau: &mut Tableau,
        entering_variable_index: usize,
    ) -> usize {
        let mut leaving_variable_index: usize = 1;

        let objective = tableau.matrix[leaving_variable_index].clone();

        let number_of_equations = tableau.row_names.len();
        let number_of_columns = tableau.column_names.len();

        let mut best_ratio = objective[number_of_columns - 1] / objective[entering_variable_index];

        for index in 1..number_of_equations - 1 {
            let constraint = tableau.matrix[index].clone();

            if constraint[entering_variable_index] > 0.0 {
                let ratio = constraint[number_of_columns - 1] / constraint[entering_variable_index];
                if ratio < best_ratio {
                    leaving_variable_index = index;
                    best_ratio = ratio;
                }
            }
        }
        leaving_variable_index
    }

    /// Iterates model one time.
    pub(self) fn recalculate_model(
        tableau: &mut Tableau,
        leaving_variable_index: usize,
        entering_variable_index: usize,
    ) {
        Solver::recalculate_pivot_row(tableau, leaving_variable_index, entering_variable_index);

        for row_index in 0..tableau.matrix.len() {
            // println!("\n---> {:?}", self.matrix.rows[row_index]);
            if row_index == leaving_variable_index {
                continue;
            }
            Solver::recalculate_row(
                tableau,
                row_index,
                leaving_variable_index,
                entering_variable_index,
            );
            // println!("---> {:?}\n", self.matrix.rows[row_index]);
        }
    }

    /// Recalculates the row.
    pub(self) fn recalculate_row(
        tableau: &mut Tableau,
        row_index: usize,
        leaving_variable_index: usize,
        entering_variable_index: usize,
    ) {
        let mut row = tableau.matrix[row_index].clone();
        let pivot_row = tableau.matrix[leaving_variable_index].clone();
        let pivot_element = tableau.matrix[row_index][entering_variable_index];

        for index in 0..row.len() {
            row[index] = row[index] - pivot_element * pivot_row[index];
        }

        tableau.matrix[row_index] = row;
    }

    /// Recalculates the pivot row.
    pub(self) fn recalculate_pivot_row(
        tableau: &mut Tableau,
        leaving_variable_index: usize,
        entering_variable_index: usize,
    ) {
        let pivot_row = [tableau.matrix[leaving_variable_index].clone()].to_vec();
        let pivot_element =
            [[1.0 / tableau.matrix[leaving_variable_index][entering_variable_index]].to_vec()]
                .to_vec();

        let new_pivot_row = matrix_product(pivot_element, pivot_row);
        tableau.matrix[leaving_variable_index] = new_pivot_row[0].clone();
    }

    /// Writes tableau to the console.
    pub(self) fn write_tableau_to_console(tableau: &Tableau) {
        let width = 5;
        let precision = 2;
        println!("\n     {:width$.precision$?}", tableau.column_names);

        for index in 0..tableau.row_names.len() {
            println!(
                "{:width$}{:width$.precision$?}",
                tableau.row_names[index], tableau.matrix[index]
            );
        }
    }

    /// Checks if the optimality condition is reached. If it is reached, returns true. Else false.
    pub(self) fn is_optimality_condition_reached(tableau: &Tableau) -> bool {
        for expression in &tableau.matrix[0] {
            if expression < &0.0 {
                return false;
            }
        }
        true
    }
}
