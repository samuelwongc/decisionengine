use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;

extern crate serde;
extern crate serde_json;

use serde_json::Value;

mod decisionengine;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut ruleset_file = File::open(&args[1]).expect("Ruleset file not found");

    let mut ruleset = String::new();
    ruleset_file
        .read_to_string(&mut ruleset)
        .expect("Something went wrong while reading the ruleset file");

    let parsed_ruleset: Vec<Value> = match serde_json::from_str(&ruleset) {
        Ok(json) => json,
        Err(error) => panic!(format!("Malformed JSON: {}", error)),
    };

    let mut rules = HashMap::new();
    for rule in &parsed_ruleset {
        let r = decisionengine::deserialize_rule(rule);
        rules.insert(r.rule_id, r);
    }

    let mut inputs_file = File::open(&args[2]).expect("Input file not found");

    let mut inputs = String::new();
    inputs_file
        .read_to_string(&mut inputs)
        .expect("Something went wrong while reading the input file");

    let parsed_inputs: Value = match serde_json::from_str(&inputs) {
        Ok(json) => json,
        Err(error) => panic!(format!("Malformed JSON: {}", error)),
    };
    let input_values = decisionengine::deserialize_inputs(&parsed_inputs);

    let result = decisionengine::eval_rules(&rules, &input_values);

    match result {
        decisionengine::EvalResult::Accept => println!("ACCEPT"),
        decisionengine::EvalResult::Reject => println!("REJECT"),
    };
}
