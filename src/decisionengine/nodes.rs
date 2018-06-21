use std::collections::HashMap;

extern crate serde_json;
use serde_json::Value;

use decisionengine::InputValue;
use decisionengine::operations::*;

pub enum NodeResult {
    Numeric(i32),
    Boolean(bool),
    Err(String)
}

pub trait EvalNode {
    fn eval(&self, input: &HashMap<String, InputValue>) -> NodeResult;
}

struct ConstantRootNode {
    value: NodeResult
}

impl EvalNode for ConstantRootNode {
    fn eval(&self, _: &HashMap<String, InputValue>) -> NodeResult {
        match &self.value {
            &NodeResult::Boolean(b) => NodeResult::Boolean(b),
            &NodeResult::Numeric(n) => NodeResult::Numeric(n),
            NodeResult::Err(msg)    => NodeResult::Err(msg.clone())
        }
    }
}

struct InputRootNode {
    value: String
}

impl EvalNode for InputRootNode {
    fn eval(&self, input: &HashMap<String, InputValue>) -> NodeResult {
        match input.get(&self.value) {
            Some(x) => match x {
                &InputValue::Boolean(b) => NodeResult::Boolean(b),
                &InputValue::Numeric(n) => NodeResult::Numeric(n)
            },
            None => NodeResult::Err(format!("Variable {} does not exist.", &self.value))
        }
    }
}

struct BinOpNode {
    lvalue: Box<EvalNode>,
    rvalue: Box<EvalNode>,
    operation: Box<BinaryOperation>
}

impl EvalNode for BinOpNode {
    fn eval(&self, input: &HashMap<String, InputValue>) -> NodeResult {
        self.operation.eval(self.lvalue.eval(input), self.rvalue.eval(input))
    }
}

pub fn deserialize_node(v: &Value) -> Box<EvalNode> {
    match v["type"].as_str().unwrap() {
        "input" => deserialize_input_node(v),
        "op" => {
            match v["op"].as_str().unwrap() {
                "pow" => deserialize_bin_op_node(v, Box::new(PowerOperation {})),
                ">="  => deserialize_bin_op_node(v, Box::new(GreaterThanOrEqualsOperation {})),
                "<="  => deserialize_bin_op_node(v, Box::new(LessThanOrEqualsOperation {})),
                "&&"  => deserialize_bin_op_node(v, Box::new(AndOperation {})),
                "+"   => deserialize_bin_op_node(v, Box::new(AdditionOperation {})),
                "=="  => deserialize_bin_op_node(v, Box::new(EqualsOperation {})),
                _     => panic!(format!("Cannot deserialize: unknown operation {}", v["op"].to_string()))
            }
        },
        "constant" => deserialize_const_node(v),
        _ => panic!(format!("Cannot deserialize node type: {}", v["type"].to_string()))
    }
}

fn deserialize_bin_op_node(v: &Value, op: Box<BinaryOperation>) -> Box<EvalNode> {
    let lvalue = deserialize_node(&v["lvalue"]);
    let rvalue = deserialize_node(&v["rvalue"]);
    Box::new(
        BinOpNode {
            lvalue: lvalue,
            rvalue: rvalue,
            operation: op
        }
    )
}

fn deserialize_const_node(v: &Value) -> Box<EvalNode>
{
    let value = v["value"].as_str();
    let root = ConstantRootNode {
        value: match value {
            Some(b) => match b { 
                "true" => NodeResult::Boolean(true),
                "false" => NodeResult::Boolean(false),
                _ => panic!(format!("Unknown value: {}", value.unwrap()))
            },
            _ => NodeResult::Numeric(v["value"].as_i64().unwrap() as i32)
        }
    };
    Box::new(root)
}

fn deserialize_input_node(v: &Value) -> Box<InputRootNode> {
    let root = InputRootNode { value: String::from(v["value"].as_str().unwrap()) };
    Box::new(root)
}