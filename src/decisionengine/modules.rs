extern crate serde_json;

use decisionengine::datasource::DecisionDataset;
use serde_json::Value;

use decisionengine::rules::deserialize_rule;
use decisionengine::{EvalResult, Evaluatable};

pub struct PassAllModule {
    pub module_name: String,
    children: Vec<Box<Evaluatable>>,
}

impl Evaluatable for PassAllModule {
    fn eval(&mut self, input: &DecisionDataset) -> EvalResult {
        println!("  Module: {} [START]", self.module_name);
        for child in &mut self.children {
            if child.eval(input) == EvalResult::Reject {
                println!("  Module: {} [REJECT]", self.module_name);
                return EvalResult::Reject;
            }
        }
        println!("  Module: {} [ACCEPT]", self.module_name);
        EvalResult::Accept
    }
}

pub fn deserialize_module(value: &Value) -> Box<Evaluatable> {
    let children = value["children"]
        .as_array()
        .unwrap()
        .into_iter()
        .map(|x| match x["type"].as_str().unwrap() {
            "rule" => Box::from(deserialize_rule(x)),
            "module" => deserialize_module(x),
            _ => panic!("Unknown module children type"),
        })
        .collect();

    let module = match value["module_type"].as_str().unwrap() {
        "all" => PassAllModule {
            module_name: value["module_name"].as_str().unwrap().to_string(),
            children: children,
        },
        _ => panic!(format!(
            "Unknown module_type: {}",
            value["module_type"].as_str().unwrap()
        )),
    };

    Box::from(module)
}
