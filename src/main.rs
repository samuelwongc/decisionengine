extern crate clap;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use clap::{App, Arg};
use std::fs::File;
use std::io::prelude::*;

mod decisionengine;
use decisionengine::Evaluatable;

fn main() {
    let matches = App::new("Decisioning Engine")
        .version("0.1alpha")
        .arg(
            Arg::with_name("ruleset")
                .help("Sets the input ruleset file to use")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("inputs")
                .short("i")
                .long("inputs")
                .value_name("INPUTS")
                .multiple(true)
                .help("Input files")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("detailed")
                .short("d")
                .long("detailed")
                .help("Evaluates entire decision tree without short-circuiting and returns result at all nodes"),
        )
        .get_matches();

    let mut decision_strategy_file =
        File::open(matches.value_of("ruleset").unwrap()).expect(&format!("Rule file not found"));

    let mut decision_module =
        decisionengine::DecisionEngine::from_file(&mut decision_strategy_file);

    let input_file_names = matches.values_of("inputs").unwrap();

    let detailed = matches.is_present("detailed");

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

        if detailed {
            let detailed_result = decisionengine::results::SubmoduleResult::ModuleResult(
                decisionengine::results::ModuleResult {
                    module_id: String::from("CBRF Silver"),
                    result: result.clone(),
                    submodule_results: Vec::new(),
                },
            );

            let mut visitor = decisionengine::visitor::ResultAggregatingVisitor {
                stack: decisionengine::visitor::ResultStack::new(detailed_result),
                input: input_values,
            };

            decision_module.accept(&mut visitor);

            println!(
                "{}",
                serde_json::to_string_pretty(visitor.stack.get_result()).unwrap()
            );
        }
    }
}
