extern crate serde_json;

use decisionengine::datasource::DecisionDataset;
use serde_json::Value;

use decisionengine::rules::deserialize_rule;
use decisionengine::{EvalResult, Evaluatable};

pub struct PassAllModule {
    children: Vec<Box<Evaluatable>>,
}

impl Evaluatable for PassAllModule {
    fn eval(&self, input: &DecisionDataset) -> EvalResult {
        for child in &self.children {
            if child.eval(input) == EvalResult::Reject {
                return EvalResult::Reject;
            }
        }
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
        "all" => PassAllModule { children: children },
        _ => panic!(format!(
            "Unknown module_type: {}",
            value["module_type"].as_str().unwrap()
        )),
    };

    Box::from(module)
}
