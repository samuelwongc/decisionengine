use std::collections::HashMap;

pub mod modules;
pub mod nodes;
pub mod operations;
pub mod rules;

#[derive(Clone, PartialEq)]
pub enum InputValue {
    Numeric(i32),
    Boolean(bool),
    Text(String),
    Array(Vec<InputValue>),
}

#[derive(PartialEq)]
pub enum EvalResult {
    Accept,
    Reject,
}

pub trait Evaluatable {
    fn eval(&self, input: &HashMap<String, InputValue>) -> EvalResult;
}
