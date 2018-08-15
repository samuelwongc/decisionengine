extern crate serde_json;

use decisionengine::datasource::DecisionDataset;
use decisionengine::rules::Rule;
use decisionengine::visitor::DecisionTreeVisitor;
use serde_json::Value;

use decisionengine::rules::deserialize_rule;
use decisionengine::{EvalResult, Evaluatable};

pub enum ModuleChildren {
    PassAllModule(PassAllModule),
    Rule(Rule),
}

trait Module {}

impl Module for PassAllModule {}

pub struct PassAllModule {
    pub module_name: String,
    pub children: Vec<ModuleChildren>,
}

impl PassAllModule {
    pub fn new(module_name: String, children: Vec<ModuleChildren>) -> Self {
        Self {
            module_name: module_name,
            children: children,
        }
    }
}

impl Evaluatable for PassAllModule {
    fn eval(&mut self, input: &DecisionDataset) -> EvalResult {
        for child in &mut self.children {
            let result = match child {
                ModuleChildren::Rule(rule) => rule.eval(input),
                ModuleChildren::PassAllModule(module) => module.eval(input),
            };
            if result == EvalResult::Reject {
                return EvalResult::Reject;
            }
        }
        EvalResult::Accept
    }

    fn accept<V: DecisionTreeVisitor>(&mut self, visitor: &mut V) {
        visitor.visit_pass_all_module(self);
        for child in &mut self.children {
            match child {
                ModuleChildren::PassAllModule(m) => {
                    m.accept(visitor);
                }
                ModuleChildren::Rule(r) => {
                    r.accept(visitor);
                }
            }
        }
        visitor.leave_pass_all_module(self);
    }
}

pub fn deserialize_module_children(value: &Value) -> ModuleChildren {
    let child_type = value["type"].as_str().unwrap();
    if child_type == "rule" {
        return ModuleChildren::Rule(deserialize_rule(value));
    } else {
        return ModuleChildren::PassAllModule(deserialize_module(value));
    }
}

pub fn deserialize_module(value: &Value) -> PassAllModule {
    let children: Vec<ModuleChildren> = value["children"]
        .as_array()
        .unwrap()
        .into_iter()
        .map(deserialize_module_children)
        .collect();

    let module = match value["module_type"].as_str().unwrap() {
        "all" => PassAllModule::new(value["module_name"].as_str().unwrap().to_string(), children),
        _ => panic!(format!(
            "Unknown module_type: {}",
            value["module_type"].as_str().unwrap()
        )),
    };

    module
}
