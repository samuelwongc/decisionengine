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

extern crate bodyparser;
extern crate iron;

use iron::prelude::*;
use iron::status;

#[derive(Serialize, Deserialize, Clone)]
struct DecisionRequest {
    application_data: decisionengine::datasource::applicationdata::ApplicationDataV1,
    decision_strategy_id: u32,
}

fn decision(req: &mut Request) -> IronResult<Response> {
    let struct_body = req.get::<bodyparser::Struct<DecisionRequest>>();
    match struct_body {
        Ok(Some(request)) => {
            let mut decision_strategy_file = File::open(format!(
                "examples/{}/ruleset.json",
                request.decision_strategy_id
            )).expect(&format!("Rule file not found"));
            let mut decision_module =
                decisionengine::DecisionEngine::from_file(&mut decision_strategy_file);

            let mut decision_dataset =
                decisionengine::datasource::DecisionDataset::new(request.application_data);

            let result = decision_module.eval(&mut decision_dataset);
            return Ok(Response::with((
                status::Ok,
                match result {
                    decisionengine::EvalResult::Accept => "ACCEPT",
                    _ => "REJECT",
                },
            )));
        }
        _ => Ok(Response::with(status::BadRequest)),
    }
}

fn server() {
    let chain = Chain::new(decision);
    Iron::new(chain).http("0.0.0.0:3000").unwrap();
}

fn cli(matches: clap::ArgMatches) {
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

        let input_application_data: decisionengine::datasource::applicationdata::ApplicationDataV1 =
            match serde_json::from_str(&inputs) {
                Ok(dd) => dd,
                _ => panic!("Cannot parse input dataset"),
            };

        let mut decision_dataset =
            decisionengine::datasource::DecisionDataset::new(input_application_data);

        let result = decision_module.eval(&mut decision_dataset);

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
                input: decision_dataset,
            };

            decision_module.accept(&mut visitor);

            println!(
                "{}",
                serde_json::to_string_pretty(visitor.stack.get_result()).unwrap()
            );
        }
    }
}

fn main() {
    let matches = App::new("Decisioning Engine")
        .version("0.1alpha")
        .arg(
            Arg::with_name("ruleset")
                .help("Sets the input ruleset file to use")
                .required(false)
                .index(1),
        )
        .arg(
            Arg::with_name("inputs")
                .short("i")
                .long("inputs")
                .value_name("INPUTS")
                .multiple(true)
                .help("Input files")
                .required(false)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("detailed")
                .short("d")
                .long("detailed")
                .help("Evaluates entire decision tree without short-circuiting and returns result at all nodes"),
        )
        .arg(
            Arg::with_name("cli")
                .short("c")
                .long("cli")
                .help("Run as command line tool."),
        )
        .get_matches();

    if !matches.is_present("cli") {
        server();
    } else {
        cli(matches);
    }
}
