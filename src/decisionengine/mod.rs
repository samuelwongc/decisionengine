extern crate serde;
extern crate serde_json;

use serde_json::Value;

pub mod datasource;
pub mod deserializers;
pub mod modules;
pub mod nodes;
pub mod operations;
pub mod results;
pub mod rules;
pub mod schema;
pub mod visitor;

use decisionengine::datasource::DecisionDataset;
use decisionengine::modules::PassAllModule;
use decisionengine::visitor::DecisionTreeVisitor;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use std::fs::File;
use std::io::prelude::*;

#[derive(PartialEq, Clone, Deserialize, Serialize)]
pub enum EvalResult {
    Accept,
    Reject,
}

pub trait Evaluatable {
    fn eval(&mut self, input: &mut DecisionDataset) -> EvalResult;
    fn accept<V: DecisionTreeVisitor>(&mut self, visitor: &mut V);
}

#[derive(Queryable)]
struct DecisionStrategy {
    decision_strategy_id: i32,
    decision_strategy_json: serde_json::Value,
}

pub struct DecisionEngine {}

impl DecisionEngine {
    pub fn from_file(file: &mut File) -> Box<PassAllModule> {
        let mut serialized_decision_strategy = String::new();
        file.read_to_string(&mut serialized_decision_strategy)
            .expect("Something went wrong while reading the decision_strategy file");

        let decision_module_json: Value = match serde_json::from_str(&serialized_decision_strategy)
        {
            Ok(json) => json,
            Err(error) => panic!(format!("Malformed JSON: {}", error)),
        };

        Box::from(self::modules::deserialize_module(&decision_module_json))
    }

    pub fn from_id(id: i32, connection: &PgConnection) -> Box<PassAllModule> {
        use decisionengine::schema::decision_strategy::dsl::*;

        let ds: DecisionStrategy = decision_strategy
            .find(id)
            .first::<DecisionStrategy>(connection)
            .expect("Error loading decision strategy");

        Box::from(self::modules::deserialize_module(
            &ds.decision_strategy_json,
        ))
    }
}
