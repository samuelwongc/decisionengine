use std::collections::HashMap;

extern crate serde_json;
use serde_json::Value;

use decisionengine::operations::*;
use decisionengine::InputValue;

#[derive(Clone, PartialEq)]
pub enum NodeResult {
    Numeric(i32),
    Boolean(bool),
    Text(String),
    Array(Vec<NodeResult>),
    Err(String),
}

pub trait EvalNode {
    fn eval(&self, input: &HashMap<String, InputValue>) -> NodeResult;
}

struct ConstantRootNode {
    value: NodeResult,
}

impl EvalNode for ConstantRootNode {
    fn eval(&self, _: &HashMap<String, InputValue>) -> NodeResult {
        match &self.value {
            &NodeResult::Boolean(b) => NodeResult::Boolean(b),
            &NodeResult::Numeric(n) => NodeResult::Numeric(n),
            &NodeResult::Text(ref s) => NodeResult::Text(s.clone()),
            &NodeResult::Array(ref a) => NodeResult::Array(a.clone()),
            NodeResult::Err(msg) => NodeResult::Err(msg.clone()),
        }
    }
}

struct InputRootNode {
    value: String,
}

fn input_value_to_node_result(value: &InputValue) -> NodeResult {
    match value {
        &InputValue::Boolean(b) => NodeResult::Boolean(b),
        &InputValue::Numeric(n) => NodeResult::Numeric(n),
        &InputValue::Text(ref s) => NodeResult::Text(s.clone()),
        &InputValue::Array(ref a) => {
            NodeResult::Array(a.into_iter().map(input_value_to_node_result).collect())
        }
    }
}

impl EvalNode for InputRootNode {
    fn eval(&self, input: &HashMap<String, InputValue>) -> NodeResult {
        match input.get(&self.value) {
            Some(x) => input_value_to_node_result(x),
            None => NodeResult::Err(format!("Variable {} does not exist.", &self.value)),
        }
    }
}

struct BinOpNode {
    lvalue: Box<EvalNode>,
    rvalue: Box<EvalNode>,
    operation: Box<BinaryOperation>,
}

impl EvalNode for BinOpNode {
    fn eval(&self, input: &HashMap<String, InputValue>) -> NodeResult {
        self.operation.eval(&self.lvalue, &self.rvalue, input)
    }
}

pub fn deserialize_node(v: &Value) -> Box<EvalNode> {
    match v["type"].as_str().unwrap() {
        "input" => deserialize_input_node(v),
        "op" => match v["op"].as_str().unwrap() {
            "pow" => deserialize_bin_op_node(v, Box::new(PowerOperation {})),
            ">=" => deserialize_bin_op_node(v, Box::new(GreaterThanOrEqualsOperation {})),
            "<=" => deserialize_bin_op_node(v, Box::new(LessThanOrEqualsOperation {})),
            "&&" => deserialize_bin_op_node(v, Box::new(AndOperation {})),
            "+" => deserialize_bin_op_node(v, Box::new(AdditionOperation {})),
            "==" => deserialize_bin_op_node(v, Box::new(EqualsOperation {})),
            "contains" => deserialize_bin_op_node(v, Box::new(ArrayContainsOperation {})),
            _ => panic!(format!(
                "Cannot deserialize: unknown operation {}",
                v["op"].to_string()
            )),
        },
        "constant" => deserialize_const_node(v),
        _ => panic!(format!(
            "Cannot deserialize node type: {}",
            v["type"].to_string()
        )),
    }
}

fn deserialize_bin_op_node(v: &Value, op: Box<BinaryOperation>) -> Box<EvalNode> {
    let lvalue = deserialize_node(&v["lvalue"]);
    let rvalue = deserialize_node(&v["rvalue"]);
    Box::new(BinOpNode {
        lvalue: lvalue,
        rvalue: rvalue,
        operation: op,
    })
}

fn deserialize_const_node_value(v: &Value) -> NodeResult {
    if v.is_array() {
        let array_value: Vec<NodeResult> = v.as_array()
            .unwrap()
            .iter()
            .map(|v| deserialize_const_node_value(v))
            .collect();
        return NodeResult::Array(array_value);
    }
    if v.is_boolean() {
        return NodeResult::Boolean(v.as_bool().unwrap());
    }
    if v.is_string() {
        return NodeResult::Text(v.as_str().unwrap().to_string());
    }
    if v.is_i64() {
        return NodeResult::Numeric(v.as_i64().unwrap() as i32);
    }
    panic!(format!(
        "Can't deserialize constant input: {}",
        v.to_string()
    ));
}

fn deserialize_const_node(v: &Value) -> Box<EvalNode> {
    let root = ConstantRootNode {
        value: deserialize_const_node_value(&v["value"]),
    };
    Box::new(root)
}

fn deserialize_input_node(v: &Value) -> Box<InputRootNode> {
    let root = InputRootNode {
        value: String::from(v["value"].as_str().unwrap()),
    };
    Box::new(root)
}
