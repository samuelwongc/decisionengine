use decisionengine::nodes::NodeResult;

pub trait BinaryOperation {
    fn eval(&self, lvalue: NodeResult, rvalue: NodeResult) -> NodeResult;
}

pub struct AdditionOperation {}

impl BinaryOperation for AdditionOperation {
    fn eval(&self, lvalue: NodeResult, rvalue: NodeResult) -> NodeResult {
        match lvalue {
            NodeResult::Numeric(l) => match rvalue {
                NodeResult::Numeric(r) => NodeResult::Numeric(l + r),
                NodeResult::Boolean(_) => NodeResult::Err(String::from("Expected int, got boolean during addition operation.")),
                e => e
            },
            NodeResult::Boolean(_) => NodeResult::Err(String::from("Expected int, got boolean during addition operation.")),
            e => e
        }
    }
}

pub struct EqualsOperation {}

impl BinaryOperation for EqualsOperation {
    fn eval(&self, lvalue: NodeResult, rvalue: NodeResult) -> NodeResult {
        match lvalue {
            NodeResult::Numeric(l) => match rvalue {
                NodeResult::Numeric(r) => NodeResult::Boolean(l == r),
                NodeResult::Boolean(_) => NodeResult::Err(String::from("Cannot compare equality between int with boolean.")),
                e => e
            },
            NodeResult::Boolean(l) => match rvalue {
                NodeResult::Numeric(_) => NodeResult::Err(String::from("Cannot compare equality between boolean with int.")),
                NodeResult::Boolean(r) => NodeResult::Boolean(l == r),
                e => e
            },
            e => e
        }
    }
}

pub struct PowerOperation {}

impl BinaryOperation for PowerOperation {
    fn eval(&self, lvalue: NodeResult, rvalue: NodeResult) -> NodeResult {
        match lvalue {
            NodeResult::Numeric(l) => match rvalue {
                NodeResult::Numeric(r) => NodeResult::Numeric(l.pow(r as u32)),
                NodeResult::Boolean(_) => NodeResult::Err(String::from("Expected int, got boolean during addition operation.")),
                e => e
            },
            NodeResult::Boolean(_) => NodeResult::Err(String::from("Expected int, got boolean during addition operation.")),
            e => e
        }
    }
}

pub struct GreaterThanOrEqualsOperation {}

impl BinaryOperation for GreaterThanOrEqualsOperation {
    fn eval(&self, lvalue: NodeResult, rvalue: NodeResult) -> NodeResult {
        match lvalue {
            NodeResult::Numeric(l) => match rvalue {
                NodeResult::Numeric(r) => NodeResult::Boolean(l >= r),
                NodeResult::Boolean(_) => NodeResult::Err(String::from("Expected int, got boolean during addition operation.")),
                e => e
            },
            NodeResult::Boolean(_) => NodeResult::Err(String::from("Expected int, got boolean during addition operation.")),
            e => e
        }
    }
}

pub struct LessThanOrEqualsOperation {}

impl BinaryOperation for LessThanOrEqualsOperation {
    fn eval(&self, lvalue: NodeResult, rvalue: NodeResult) -> NodeResult {
        match lvalue {
            NodeResult::Numeric(l) => match rvalue {
                NodeResult::Numeric(r) => NodeResult::Boolean(l <= r),
                NodeResult::Boolean(_) => NodeResult::Err(String::from("Expected int, got boolean during addition operation.")),
                e => e
            },
            NodeResult::Boolean(_) => NodeResult::Err(String::from("Expected int, got boolean during addition operation.")),
            e => e
        }
    }
}

pub struct AndOperation {}

impl BinaryOperation for AndOperation {
    fn eval(&self, lvalue: NodeResult, rvalue: NodeResult) -> NodeResult {
        match lvalue {
            NodeResult::Boolean(true) => match rvalue {
                NodeResult::Boolean(b) => NodeResult::Boolean(b),
                NodeResult::Numeric(_) => NodeResult::Err(String::from("Expected bool, got int during AND operation.")),
                e => e
            },
            NodeResult::Boolean(false) => NodeResult::Boolean(false),
            NodeResult::Numeric(_) => NodeResult::Err(String::from("Expected bool, got int during AND operation.")),
            e => e
        }
    }
}