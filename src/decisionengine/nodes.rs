use decisionengine::datasource::deserialize_input_node;
use decisionengine::datasource::DecisionDataset;

extern crate serde_json;
use serde_json::Value;

use decisionengine::operations::*;

#[derive(Clone, PartialEq)]
pub enum NodeResult {
    Numeric(i32),
    Boolean(bool),
    Text(String),
    Array(Vec<NodeResult>),
    Err(String),
}

pub trait EvalNode {
    fn eval(&mut self, input: &mut DecisionDataset) -> NodeResult;
}

struct ConstantRootNode {
    value: NodeResult,
}

impl EvalNode for ConstantRootNode {
    fn eval(&mut self, _: &mut DecisionDataset) -> NodeResult {
        match &self.value {
            &NodeResult::Boolean(b) => NodeResult::Boolean(b),
            &NodeResult::Numeric(n) => NodeResult::Numeric(n),
            &NodeResult::Text(ref s) => NodeResult::Text(s.clone()),
            &NodeResult::Array(ref a) => NodeResult::Array(a.clone()),
            NodeResult::Err(msg) => NodeResult::Err(msg.clone()),
        }
    }
}

struct BinOpNode {
    lvalue: Box<EvalNode>,
    rvalue: Box<EvalNode>,
    operation: Box<BinaryOperation>,
}

impl EvalNode for BinOpNode {
    fn eval(&mut self, input: &mut DecisionDataset) -> NodeResult {
        self.operation
            .eval(&mut self.lvalue, &mut self.rvalue, input)
    }
}

pub fn deserialize_node(v: &Value) -> (Box<EvalNode>, bool) {
    let node_type = v["type"].as_str().unwrap();
    if node_type == "constant" {
        deserialize_const_node(v)
    } else if node_type == "input" {
        deserialize_input_node(v["value"].as_str().unwrap())
    } else {
        match v["type"].as_str().unwrap() {
            "op" => match v["op"].as_str().unwrap() {
                "pow" => deserialize_bin_op_node(v, Box::new(PowerOperation {})),
                ">=" => deserialize_bin_op_node(v, Box::new(GreaterThanOrEqualsOperation {})),
                "<=" => deserialize_bin_op_node(v, Box::new(LessThanOrEqualsOperation {})),
                "&&" => deserialize_bin_op_node(v, Box::new(AndOperation {})),
                "+" => deserialize_bin_op_node(v, Box::new(AdditionOperation {})),
                "==" => deserialize_bin_op_node(v, Box::new(EqualsOperation {})),
                "array_contains" => deserialize_bin_op_node(v, Box::new(ArrayContainsOperation {})),
                "regex_contains" => deserialize_bin_op_node(v, Box::new(RegexContainsOperation {})),
                _ => panic!(format!(
                    "Cannot deserialize: unknown operation {}",
                    v["op"].to_string()
                )),
            },
            _ => panic!(format!(
                "Cannot deserialize node type: {}",
                v["type"].to_string()
            )),
        }
    }
}

fn deserialize_bin_op_node(v: &Value, op: Box<BinaryOperation>) -> (Box<EvalNode>, bool) {
    let (mut lvalue, lconst) = deserialize_node(&v["lvalue"]);
    let (mut rvalue, rconst) = deserialize_node(&v["rvalue"]);
    if lconst && rconst {
        (
            Box::new(ConstantRootNode {
                value: op.eval(&mut lvalue, &mut rvalue, &mut DecisionDataset::get_empty()),
            }),
            true,
        )
    } else {
        (
            Box::new(BinOpNode {
                lvalue: lvalue,
                rvalue: rvalue,
                operation: op,
            }),
            false,
        )
    }
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

fn deserialize_const_node(v: &Value) -> (Box<EvalNode>, bool) {
    let root = ConstantRootNode {
        value: deserialize_const_node_value(&v["value"]),
    };
    (Box::new(root), true)
}
