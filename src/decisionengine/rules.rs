use std::collections::HashMap;

extern crate serde_json;
use serde_json::Value;

use decisionengine::nodes::*;
use decisionengine::{EvalResult, Evaluatable, InputValue};

pub struct Rule {
    pub rule_id: i32,
    conditions: HashMap<i32, Condition>,
}

impl Evaluatable for Rule {
    fn eval(&self, input: &HashMap<String, InputValue>) -> EvalResult {
        let mut curr_condition_id = 1;
        loop {
            let result = match &self.conditions.get(&curr_condition_id) {
                Some(condition) => condition.eval(input),
                _ => panic!("Condition not found."),
            };
            match result {
                ConditionResult::Accept => return EvalResult::Accept,
                ConditionResult::Reject => return EvalResult::Reject,
                &ConditionResult::Condition(condition_id) => curr_condition_id = condition_id,
            }
        }
    }
}

enum ConditionResult {
    Accept,
    Reject,
    Condition(i32),
}

pub struct Condition {
    pub condition_id: i32,
    node: Box<EvalNode>,
    if_true: ConditionResult,
    if_false: ConditionResult,
}

impl Condition {
    fn eval(&self, input: &HashMap<String, InputValue>) -> &ConditionResult {
        match self.node.eval(input) {
            NodeResult::Boolean(b) => if b {
                &self.if_true
            } else {
                &self.if_false
            },
            NodeResult::Err(msg) => panic!(msg),
            _ => panic!("Top level node in condition must return bool."),
        }
    }
}

pub fn deserialize_rule(v: &Value) -> Rule {
    let mut conditions = HashMap::new();
    for condition in v["conditions"].as_array().unwrap() {
        let r = deserialize_condition(condition);
        conditions.insert(r.condition_id, r);
    }

    Rule {
        rule_id: v["rule_id"].as_i64().unwrap() as i32,
        conditions: conditions,
    }
}

fn deserialize_condition_decision(v: &Value) -> ConditionResult {
    match v["type"].as_str().unwrap() {
        "return" => match v["value"].as_str().unwrap() {
            "ACCEPT" => ConditionResult::Accept,
            "REJECT" => ConditionResult::Reject,
            _ => panic!("Unknown condition decision."),
        },
        "goto" => ConditionResult::Condition(v["value"].as_str().unwrap().parse::<i32>().unwrap()),
        _ => panic!("Unknown condition decision."),
    }
}

pub fn deserialize_condition(v: &Value) -> Condition {
    let (node, _) = deserialize_node(&v["condition"]);
    Condition {
        condition_id: v["condition_id"].as_str().unwrap().parse::<i32>().unwrap(),
        node: node,
        if_true: deserialize_condition_decision(&v["true"]),
        if_false: deserialize_condition_decision(&v["false"]),
    }
}

fn deserialize_input(v: &Value) -> InputValue {
    if v.is_array() {
        let array_value: Vec<InputValue> = v.as_array()
            .unwrap()
            .iter()
            .map(|v| deserialize_input(v))
            .collect();
        return InputValue::Array(array_value);
    }
    if v.is_boolean() {
        return InputValue::Boolean(v.as_bool().unwrap());
    }
    if v.is_string() {
        return InputValue::Text(v.as_str().unwrap().to_string());
    }
    return InputValue::Numeric(v.as_i64().unwrap() as i32);
}

pub fn deserialize_inputs(v: &Value) -> HashMap<String, InputValue> {
    let raw_inputs = v.as_object().unwrap();

    let mut inputs = HashMap::new();
    for (k, v) in raw_inputs.iter() {
        inputs.insert(k.clone(), deserialize_input(v));
    }
    inputs
}
