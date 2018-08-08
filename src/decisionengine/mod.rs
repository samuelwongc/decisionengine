use std::collections::HashMap;

extern crate serde_json;
use serde_json::Value;

pub mod nodes;
pub mod operations;
use self::nodes::*;

#[derive(Clone, PartialEq)]
pub enum InputValue {
    Numeric(i32),
    Boolean(bool),
    Text(String),
    Array(Vec<InputValue>),
}

pub enum EvalResult {
    Accept,
    Reject,
}

enum RuleDecision {
    Accept,
    Reject,
    Rule(i32),
}

pub struct Rule {
    pub rule_id: i32,
    node: Box<EvalNode>,
    if_true: RuleDecision,
    if_false: RuleDecision,
}

impl Rule {
    fn eval(&self, input: &HashMap<String, InputValue>) -> &RuleDecision {
        match self.node.eval(input) {
            NodeResult::Boolean(b) => if b {
                &self.if_true
            } else {
                &self.if_false
            },
            NodeResult::Err(msg) => panic!(msg),
            _ => panic!("Top level node in rule must return bool."),
        }
    }
}

pub fn eval_rules(rules: &HashMap<i32, Rule>, input: &HashMap<String, InputValue>) -> EvalResult {
    let mut curr_rule_id = 1;
    loop {
        let result = match rules.get(&curr_rule_id) {
            Some(rule) => rule.eval(input),
            _ => panic!("Rule not found."),
        };
        match result {
            RuleDecision::Accept => return EvalResult::Accept,
            RuleDecision::Reject => return EvalResult::Reject,
            &RuleDecision::Rule(rule_id) => curr_rule_id = rule_id,
        }
    }
}

fn deserialize_rule_decision(v: &Value) -> RuleDecision {
    match v["type"].as_str().unwrap() {
        "return" => match v["value"].as_str().unwrap() {
            "ACCEPT" => RuleDecision::Accept,
            "REJECT" => RuleDecision::Reject,
            _ => panic!("Unknown rule decision."),
        },
        "goto" => RuleDecision::Rule(v["value"].as_str().unwrap().parse::<i32>().unwrap()),
        _ => panic!("Unknown rule decision."),
    }
}

pub fn deserialize_rule(v: &Value) -> Rule {
    Rule {
        rule_id: v["rule_id"].as_str().unwrap().parse::<i32>().unwrap(),
        node: deserialize_node(&v["rule"]),
        if_true: deserialize_rule_decision(&v["true"]),
        if_false: deserialize_rule_decision(&v["false"]),
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
