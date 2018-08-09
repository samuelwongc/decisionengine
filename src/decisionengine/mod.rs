use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

extern crate serde_json;
use serde_json::Value;

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

pub struct DecisionEngine {}

impl DecisionEngine {
    pub fn from_file(file: &mut File) -> Box<Evaluatable> {
        let mut decision_strategy = String::new();
        file.read_to_string(&mut decision_strategy)
            .expect("Something went wrong while reading the decision_strategy file");

        let decision_module_json: Value = match serde_json::from_str(&decision_strategy) {
            Ok(json) => json,
            Err(error) => panic!(format!("Malformed JSON: {}", error)),
        };

        Box::from(self::modules::deserialize_module(&decision_module_json))
    }
}
