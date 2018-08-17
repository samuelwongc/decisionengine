extern crate bodyparser;
extern crate clap;
#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate iron;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate router;
extern crate serde_json;

use clap::{App, Arg};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use iron::mime::Mime;
use iron::prelude::*;
use iron::status;
use router::Router;
use std::env;
use std::fs::File;
use std::io::prelude::*;

mod decisionengine;
use decisionengine::Evaluatable;

#[derive(Serialize, Deserialize, Clone)]
struct DecisionRequest {
    application_data: decisionengine::datasource::applicationdata::ApplicationDataV1,
    decision_strategy_id: i32,
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

fn create_decision_strategy(req: &mut Request) -> IronResult<Response> {
    let connection = establish_connection();

    let body = req.get::<bodyparser::Raw>();
    let content_type = "application/json".parse::<Mime>().unwrap();

    match body {
        Ok(Some(json)) => {
            let decision_strategy_json = serde_json::from_str(&json);
            let decision_strategy = decisionengine::DecisionStrategy::create(
                decision_strategy_json.unwrap(),
                &connection,
            );
            return Ok(Response::with((
                content_type,
                status::Ok,
                format!(
                    "{{\"decision_strategy_id\": {}}}",
                    decision_strategy.decision_strategy_id()
                ),
            )));
        }
        _ => Ok(Response::with(status::BadRequest)),
    }
}

fn decision(req: &mut Request) -> IronResult<Response> {
    let connection = establish_connection();

    let struct_body = req.get::<bodyparser::Struct<DecisionRequest>>();
    match struct_body {
        Ok(Some(request)) => {
            let mut decision_module = decisionengine::DecisionStrategy::from_id(
                request.decision_strategy_id,
                &connection,
            ).get_module();

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
    let mut router = Router::new();
    router.post("/decision", decision, "decision");
    router.post(
        "/decisionstrategy",
        create_decision_strategy,
        "decision_strategy_crate",
    );

    Iron::new(router).http("0.0.0.0:3000").unwrap();
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
