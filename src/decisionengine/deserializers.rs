use decisionengine::modules::{ModuleChildren, PassAllModule};
use decisionengine::rules::{Condition, Rule};
use serde_json::Value;
use std::collections::HashMap;

trait Deserializer {
    fn deserialize_rule(&self, value: &Value) -> Rule;
    fn deserialize_module(&self, value: &Value) -> PassAllModule;
}

pub struct DefaultDeserializer {}

impl Deserializer for DefaultDeserializer {
    fn deserialize_module(&self, value: &Value) -> PassAllModule {
        let children = value["children"]
            .as_array()
            .unwrap()
            .into_iter()
            .map(|x| match x["type"].as_str().unwrap() {
                "rule" => ModuleChildren::Rule(self.deserialize_rule(x)),
                "module" => ModuleChildren::PassAllModule(self.deserialize_module(x)),
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
        module
    }

    fn deserialize_rule(&self, value: &Value) -> Rule {
        let mut conditions = HashMap::new();
        for condition in value["conditions"].as_array().unwrap() {
            let r = Condition::deserialize(condition);
            conditions.insert(r.condition_id, r);
        }

        Rule {
            rule_name: value["rule_name"].as_str().unwrap().to_string(),
            rule_id: value["rule_id"].as_i64().unwrap() as i32,
            conditions: conditions,
        }
    }
}
