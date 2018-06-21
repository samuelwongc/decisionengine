
use std::collections::HashMap;

extern crate serde_json;
use serde_json::Value;

pub mod nodes;
pub mod operations;

use self::nodes::*;

#[derive(Copy, Clone)]
pub enum InputValue {
    Numeric(i32),
    Boolean(bool)
}

enum RuleDecision {
    ACCEPT,
    REJECT,
    Rule(i32)
}

pub struct Rule {
    pub rule_id: i32,
    node: Box<EvalNode>,
    if_true: RuleDecision,
    if_false: RuleDecision
}

impl Rule {
    fn eval(&self, input: &HashMap<String, InputValue>) -> &RuleDecision {
        match self.node.eval(input) {
            NodeResult::Boolean(b) => if b { &self.if_true } else { &self.if_false},
            NodeResult::Numeric(_) => panic!("Top level node in rule must return bool, int found."),
            NodeResult::Err(msg)   => panic!(msg)
        }
    }
}

pub fn eval_rules(rules: &HashMap<i32, Rule>, input: &HashMap<String, InputValue>) {
    let mut curr_rule_id = 1;
    loop {
        let result = match rules.get(&curr_rule_id) {
            Some(rule) => rule.eval(input),
            _          => panic!("Rule not found.")
        };
        match result {
            RuleDecision::ACCEPT => {
                println!("ACCEPT");
                break;
            },
            RuleDecision::REJECT => {
                println!("REJECT");
                break;
            },
            &RuleDecision::Rule(rule_id) => {
                curr_rule_id = rule_id;
            }
        }
    }
}

fn deserialize_rule_decision(v: &Value) -> RuleDecision {
    match v["type"].as_str().unwrap() {
        "return" => match v["value"].as_str().unwrap() {
            "ACCEPT" => RuleDecision::ACCEPT,
            "REJECT" => RuleDecision::REJECT,
            _        => panic!("Unknown rule decision.")
        },
        "goto" => RuleDecision::Rule(v["value"].as_str().unwrap().parse::<i32>().unwrap()),
        _ => panic!("Unknown rule decision.")
    }
}

pub fn deserialize_rule(v: &Value) -> Rule {
    Rule {
        rule_id: v["rule_id"].as_str().unwrap().parse::<i32>().unwrap(),
        node: deserialize_node(&v["rule"]),
        if_true: deserialize_rule_decision(&v["true"]),
        if_false: deserialize_rule_decision(&v["false"])
    }
}


pub fn deserialize_inputs(v: &Value) -> HashMap<String, InputValue> {
    let raw_inputs = v.as_object().unwrap();

    let mut inputs = HashMap::new();
    for (k, v) in raw_inputs.iter() {
        let input_value = match v.as_str() {
            Some(s) => match s{
                "false" => InputValue::Boolean(false),
                "true" => InputValue::Boolean(true),
                _ => panic!(format!("Unknown value: {}", s))
            },
            None => InputValue::Numeric(v.as_i64().unwrap() as i32)
        };
        inputs.insert(k.clone(), input_value);
    }
    inputs
}
