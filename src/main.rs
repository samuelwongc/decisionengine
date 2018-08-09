#[macro_use]
extern crate serde_derive;

use std::env;
use std::fs::File;
use std::io::prelude::*;

extern crate serde;
extern crate serde_json;

mod decisionengine;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut decision_strategy_file =
        File::open(&args[1]).expect(&format!("File {} not found", &args[1]));

    let decision_module = decisionengine::DecisionEngine::from_file(&mut decision_strategy_file);

    let input_file_names = args.get(2..args.len()).unwrap();

    for input_file_name in input_file_names {
        let mut input_file =
            File::open(input_file_name).expect(&format!("File {} not found.", input_file_name));

        let mut inputs = String::new();
        input_file
            .read_to_string(&mut inputs)
            .expect("Something went wrong while reading the input dataset file");

        let input_values: decisionengine::datasource::DecisionDataset =
            match serde_json::from_str(&inputs) {
                Ok(dd) => dd,
                _ => panic!("Cannot parse input dataset"),
            };

        let result = decision_module.eval(&input_values);

        match result {
            decisionengine::EvalResult::Accept => println!("{} [ACCEPT]", input_file_name),
            decisionengine::EvalResult::Reject => println!("{} [REJECT]", input_file_name),
        };
    }
}
