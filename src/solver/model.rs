use super::structs::{Model, Expression};

impl Model {

    pub(crate) fn add_slack_variables(&mut self) -> Vec<String> {
        let mut row_names = vec![];
        row_names.push("Z".to_string());
        let mut slack_counter = 0;
        for constraint in &mut self.constraints {
            if constraint.operator == "<=" {
                slack_counter += 1;
                let slack_name = "S_".to_string() + &slack_counter.to_string();
                row_names.push(slack_name.clone());
                let slack = Expression { coefficient: 1.0, variable: slack_name};
                constraint.left_side.push(slack);
            }
        }
        row_names
    }

    pub(crate) fn get_all_variable_names(&mut self) -> Vec<String> {
        let mut column_names: Vec<String> = vec![];
        for expression in self.objective.left_side.clone() {
            column_names.push(expression.variable);
        }
        for constraint in self.constraints.clone() {
            for expression in constraint.left_side.clone() {
                if !column_names.contains(&expression.variable) {
                    column_names.push(expression.variable);
                }
            }
        }
        column_names.push("Sol".to_string());
        column_names
    }

    pub(crate) fn create_matrix(&self, column_names: Vec<String>) -> Vec<Vec<f64>> {
        let mut matrix = vec![];
        let mut row = vec![];
        for index in 0..column_names.len() - 1 {
            for expression_index in 0..self.objective.left_side.clone().len() {
                if column_names[index] == self.objective.left_side[expression_index].variable {
                    row.push(self.objective.left_side[expression_index].coefficient * -1.0);
                    break;
                }
                if expression_index == self.objective.left_side.clone().len() - 1 {
                    row.push(0.0);
                }
            }
        }
        row.push(0.0);
        matrix.push(row);
        for constraint in self.constraints.clone() {
            let mut row = vec![];
            for index in 0..column_names.len() - 1 {
                for expression_index in 0..constraint.left_side.clone().len() {
                    if column_names[index] == constraint.left_side[expression_index].variable {
                        row.push(constraint.left_side[expression_index].coefficient);
                        break;
                    }
                    if expression_index == constraint.left_side.clone().len() - 1 {
                        row.push(0.0);
                    }
                }
            }
            row.push(constraint.right_side);
            matrix.push(row);
        }
        matrix
    }
    
}