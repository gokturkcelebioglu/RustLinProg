use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct Expression {
    pub(crate) coefficient: f64,
    pub(crate) variable: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct Objective {
    pub(crate) left_side: Vec<Expression>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct Constraint {
    pub(crate) left_side: Vec<Expression>,
    pub(crate) right_side: f64,
    pub(crate) operator: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Model {
    pub(crate) objective: Objective,
    pub(crate) constraints: Vec<Constraint>,
    pub(crate) objective_type: String,
}