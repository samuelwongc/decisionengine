extern crate regex;

use self::regex::Regex;
use decisionengine::datasource::DecisionDataset;
use decisionengine::nodes::{EvalNode, NodeResult};

pub trait BinaryOperation {
    fn eval(
        &self,
        lnode: &mut Box<EvalNode>,
        rnode: &mut Box<EvalNode>,
        inputs: &mut DecisionDataset,
    ) -> NodeResult;
}

pub struct AdditionOperation {}

impl BinaryOperation for AdditionOperation {
    fn eval(
        &self,
        lnode: &mut Box<EvalNode>,
        rnode: &mut Box<EvalNode>,
        inputs: &mut DecisionDataset,
    ) -> NodeResult {
        match lnode.eval(inputs) {
            NodeResult::Numeric(l) => match rnode.eval(inputs) {
                NodeResult::Numeric(r) => NodeResult::Numeric(l + r),
                NodeResult::Boolean(_) => NodeResult::Err(String::from(
                    "Expected int, got boolean during addition operation.",
                )),
                e => e,
            },
            NodeResult::Boolean(_) => NodeResult::Err(String::from(
                "Expected int, got boolean during addition operation.",
            )),
            e => e,
        }
    }
}

pub struct EqualsOperation {}

impl BinaryOperation for EqualsOperation {
    fn eval(
        &self,
        lnode: &mut Box<EvalNode>,
        rnode: &mut Box<EvalNode>,
        inputs: &mut DecisionDataset,
    ) -> NodeResult {
        match lnode.eval(inputs) {
            NodeResult::Numeric(l) => match rnode.eval(inputs) {
                NodeResult::Numeric(r) => NodeResult::Boolean(l == r),
                NodeResult::Boolean(_) => NodeResult::Err(String::from(
                    "Cannot compare equality between int with boolean.",
                )),
                e => e,
            },
            NodeResult::Boolean(l) => match rnode.eval(inputs) {
                NodeResult::Numeric(_) => NodeResult::Err(String::from(
                    "Cannot compare equality between boolean with int.",
                )),
                NodeResult::Boolean(r) => NodeResult::Boolean(l == r),
                e => e,
            },
            e => e,
        }
    }
}

pub struct PowerOperation {}

impl BinaryOperation for PowerOperation {
    fn eval(
        &self,
        lnode: &mut Box<EvalNode>,
        rnode: &mut Box<EvalNode>,
        inputs: &mut DecisionDataset,
    ) -> NodeResult {
        match lnode.eval(inputs) {
            NodeResult::Numeric(l) => match rnode.eval(inputs) {
                NodeResult::Numeric(r) => NodeResult::Numeric(l.pow(r as u32)),
                NodeResult::Boolean(_) => NodeResult::Err(String::from(
                    "Expected int, got boolean during addition operation.",
                )),
                e => e,
            },
            NodeResult::Boolean(_) => NodeResult::Err(String::from(
                "Expected int, got boolean during addition operation.",
            )),
            e => e,
        }
    }
}

pub struct GreaterThanOrEqualsOperation {}

impl BinaryOperation for GreaterThanOrEqualsOperation {
    fn eval(
        &self,
        lnode: &mut Box<EvalNode>,
        rnode: &mut Box<EvalNode>,
        inputs: &mut DecisionDataset,
    ) -> NodeResult {
        match lnode.eval(inputs) {
            NodeResult::Numeric(l) => match rnode.eval(inputs) {
                NodeResult::Numeric(r) => NodeResult::Boolean(l >= r),
                NodeResult::Boolean(_) => NodeResult::Err(String::from(
                    "Expected int, got boolean during addition operation.",
                )),
                e => e,
            },
            NodeResult::Boolean(_) => NodeResult::Err(String::from(
                "Expected int, got boolean during addition operation.",
            )),
            e => e,
        }
    }
}

pub struct LessThanOrEqualsOperation {}

impl BinaryOperation for LessThanOrEqualsOperation {
    fn eval(
        &self,
        lnode: &mut Box<EvalNode>,
        rnode: &mut Box<EvalNode>,
        inputs: &mut DecisionDataset,
    ) -> NodeResult {
        match lnode.eval(inputs) {
            NodeResult::Numeric(l) => match rnode.eval(inputs) {
                NodeResult::Numeric(r) => NodeResult::Boolean(l <= r),
                NodeResult::Boolean(_) => NodeResult::Err(String::from(
                    "Expected int, got boolean during addition operation.",
                )),
                e => e,
            },
            NodeResult::Boolean(_) => NodeResult::Err(String::from(
                "Expected int, got boolean during addition operation.",
            )),
            e => e,
        }
    }
}

pub struct AndOperation {}

impl BinaryOperation for AndOperation {
    fn eval(
        &self,
        lnode: &mut Box<EvalNode>,
        rnode: &mut Box<EvalNode>,
        inputs: &mut DecisionDataset,
    ) -> NodeResult {
        match lnode.eval(inputs) {
            NodeResult::Boolean(true) => match rnode.eval(inputs) {
                NodeResult::Boolean(b) => NodeResult::Boolean(b),
                NodeResult::Numeric(_) => {
                    NodeResult::Err(String::from("Expected bool, got int during AND operation."))
                }
                e => e,
            },
            NodeResult::Boolean(false) => NodeResult::Boolean(false),
            NodeResult::Numeric(_) => {
                NodeResult::Err(String::from("Expected bool, got int during AND operation."))
            }
            e => e,
        }
    }
}

pub struct ArrayContainsOperation {}

impl BinaryOperation for ArrayContainsOperation {
    fn eval(
        &self,
        lnode: &mut Box<EvalNode>,
        rnode: &mut Box<EvalNode>,
        inputs: &mut DecisionDataset,
    ) -> NodeResult {
        match lnode.eval(inputs) {
            NodeResult::Array(v) => NodeResult::Boolean(v.contains(&rnode.eval(inputs))),
            _ => NodeResult::Err(String::from(
                "lvalue of array_contains operation is not an array.",
            )),
        }
    }
}

pub struct RegexContainsOperation {}

impl BinaryOperation for RegexContainsOperation {
    fn eval(
        &self,
        lnode: &mut Box<EvalNode>,
        rnode: &mut Box<EvalNode>,
        inputs: &mut DecisionDataset,
    ) -> NodeResult {
        match lnode.eval(inputs) {
            NodeResult::Text(t) => match rnode.eval(inputs) {
                NodeResult::Text(pattern) => {
                    let p = Regex::new(&pattern);
                    match p {
                        Ok(p) => NodeResult::Boolean(p.is_match(&t)),
                        _ => NodeResult::Err(format!("Invalid regular expression {}", pattern)),
                    }
                }
                _ => NodeResult::Err(String::from(
                    "rvalue of regex_contains operation is not an string.",
                )),
            },
            _ => NodeResult::Err(String::from(
                "lvalue of array_contains operation is not an string.",
            )),
        }
    }
}
