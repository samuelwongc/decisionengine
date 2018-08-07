use decisionengine::nodes::{EvalNode, NodeResult};
use decisionengine::InputValue;
use std::collections::HashMap;

pub trait BinaryOperation {
    fn eval(
        &self,
        lnode: &Box<EvalNode>,
        rnode: &Box<EvalNode>,
        inputs: &HashMap<String, InputValue>,
    ) -> NodeResult;
}

pub struct AdditionOperation {}

impl BinaryOperation for AdditionOperation {
    fn eval(
        &self,
        lnode: &Box<EvalNode>,
        rnode: &Box<EvalNode>,
        inputs: &HashMap<String, InputValue>,
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
        lnode: &Box<EvalNode>,
        rnode: &Box<EvalNode>,
        inputs: &HashMap<String, InputValue>,
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
        lnode: &Box<EvalNode>,
        rnode: &Box<EvalNode>,
        inputs: &HashMap<String, InputValue>,
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
        lnode: &Box<EvalNode>,
        rnode: &Box<EvalNode>,
        inputs: &HashMap<String, InputValue>,
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
        lnode: &Box<EvalNode>,
        rnode: &Box<EvalNode>,
        inputs: &HashMap<String, InputValue>,
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
        lnode: &Box<EvalNode>,
        rnode: &Box<EvalNode>,
        inputs: &HashMap<String, InputValue>,
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
