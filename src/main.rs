use std::env;
use std::fs::File;
use std::io::prelude::*;

extern crate serde;
extern crate serde_json;

use serde_json::Value;

mod decisionengine;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut decision_strategy_file =
        File::open(&args[1]).expect("decision_strategy file not found");

    let mut decision_strategy = String::new();
    decision_strategy_file
        .read_to_string(&mut decision_strategy)
        .expect("Something went wrong while reading the decision_strategy file");

    let decision_module_json: Value = match serde_json::from_str(&decision_strategy) {
        Ok(json) => json,
        Err(error) => panic!(format!("Malformed JSON: {}", error)),
    };

    let decision_module = decisionengine::modules::deserialize_module(&decision_module_json);

    let mut inputs_file = File::open(&args[2]).expect("Input file not found");

    let mut inputs = String::new();
    inputs_file
        .read_to_string(&mut inputs)
        .expect("Something went wrong while reading the input file");

    let parsed_inputs: Value = match serde_json::from_str(&inputs) {
        Ok(json) => json,
        Err(error) => panic!(format!("Malformed JSON: {}", error)),
    };

    let input_values = decisionengine::rules::deserialize_inputs(&parsed_inputs);

    let result = decision_module.eval(&input_values);

    match result {
        decisionengine::EvalResult::Accept => println!("ACCEPT"),
        decisionengine::EvalResult::Reject => println!("REJECT"),
    };
}
