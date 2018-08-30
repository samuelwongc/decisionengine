extern crate serde_json;

use decisionengine::datasource::DecisionDataset;
use decisionengine::nodes::deserialize_node;
use decisionengine::nodes::EvalNode;
use decisionengine::nodes::NodeResult;
use decisionengine::rules::Rule;
use decisionengine::visitor::DecisionTreeVisitor;
use serde_json::Value;

use decisionengine::rules::deserialize_rule;
use decisionengine::{EvalResult, Evaluatable};

pub enum ModuleChildren {
    SimpleModule(SimpleModule),
    PassAllModule(PassAllModule),
    Rule(Rule),
}

pub trait Module: Evaluatable {
    fn module_name(&self) -> String;
}

impl Module for PassAllModule {
    fn module_name(&self) -> String {
        self.module_name.clone()
    }
}

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

pub struct Variable {
    pub name: String,
    pub definition: Box<EvalNode>,
    pub value: Option<NodeResult>,
}

impl Variable {
    pub fn parse_node(parts: &Vec<&str>) -> InputNode {
        InputNode {
            variable: String::from(parts[0]),
        }
    }

    pub fn deserialize(value: &serde_json::Value) -> Self {
        Variable {
            name: value["name"].as_str().unwrap().to_string(),
            definition: deserialize_node(&value["definition"]).0,
            value: None,
        }
    }
}

impl EvalNode for Variable {
    fn eval(&mut self, input: &mut DecisionDataset) -> NodeResult {
        if self.value.is_none() {
            self.value = Some(self.definition.eval(input))
        }
        self.value.clone().unwrap()
    }
}

pub struct InputNode {
    pub variable: String,
}

impl EvalNode for InputNode {
    fn eval(&mut self, input: &mut DecisionDataset) -> NodeResult {
        input.get_variables().get(&self.variable).unwrap().clone()
    }
}

pub struct SimpleModule {
    pub module_name: String,
    pub children: Vec<ModuleChildren>,
    pub accept_strategy: Box<ModuleStrategy>,
    pub variables: Vec<Variable>,
}

impl Module for SimpleModule {
    fn module_name(&self) -> String {
        self.module_name.clone()
    }
}

pub trait ModuleStrategy {
    // Compute whether reject based on partial evaluation of
    // a module since it is possible to prematurely reject
    // via short circuiting
    fn reject(&self, &Vec<EvalResult>) -> bool;
}

struct PassAllModuleStrategy {}

struct FailAtMostXModuleStrategy {
    limit: i32,
}

impl FailAtMostXModuleStrategy {
    pub fn new(limit: i32) -> Self {
        Self { limit: limit }
    }
}

impl ModuleStrategy for FailAtMostXModuleStrategy {
    fn reject(&self, results: &Vec<EvalResult>) -> bool {
        results
            .into_iter()
            .map(|x| match x {
                EvalResult::Accept => 0,
                EvalResult::Reject => 1,
            })
            .fold(0, |x, y| x + y) > self.limit
    }
}

impl ModuleStrategy for PassAllModuleStrategy {
    fn reject(&self, results: &Vec<EvalResult>) -> bool {
        !results
            .into_iter()
            .map(|x| match x {
                EvalResult::Accept => true,
                EvalResult::Reject => false,
            })
            .fold(true, |x, y| x && y)
    }
}

impl SimpleModule {
    pub fn new(
        module_name: String,
        children: Vec<ModuleChildren>,
        accept_strategy: Box<ModuleStrategy>,
        variables: Vec<Variable>,
    ) -> Self {
        Self {
            module_name: module_name,
            children: children,
            accept_strategy: accept_strategy,
            variables: variables,
        }
    }
}

impl Evaluatable for SimpleModule {
    fn eval(&mut self, input: &mut DecisionDataset) -> EvalResult {
        for variable in &mut self.variables {
            let k = variable.eval(input);
            input.get_variables().insert(variable.name.clone(), k);
        }

        let mut child_results: Vec<EvalResult> = Vec::new();
        for child in &mut self.children {
            let result = match child {
                ModuleChildren::Rule(rule) => rule.eval(input),
                ModuleChildren::PassAllModule(module) => module.eval(input),
                ModuleChildren::SimpleModule(module) => module.eval(input),
            };
            child_results.push(result);
            if self.accept_strategy.reject(&child_results) {
                return EvalResult::Reject;
            }
        }
        EvalResult::Accept
    }

    fn accept<V: DecisionTreeVisitor>(&mut self, visitor: &mut V) {
        visitor.visit_module(self);
        for child in &mut self.children {
            match child {
                ModuleChildren::PassAllModule(m) => {
                    m.accept(visitor);
                }
                ModuleChildren::SimpleModule(m) => {
                    m.accept(visitor);
                }
                ModuleChildren::Rule(r) => {
                    r.accept(visitor);
                }
            }
        }
        visitor.leave_module(self);
    }
}

impl Evaluatable for PassAllModule {
    fn eval(&mut self, input: &mut DecisionDataset) -> EvalResult {
        for child in &mut self.children {
            let result = match child {
                ModuleChildren::Rule(rule) => rule.eval(input),
                ModuleChildren::PassAllModule(module) => module.eval(input),
                ModuleChildren::SimpleModule(module) => module.eval(input),
            };
            if result == EvalResult::Reject {
                return EvalResult::Reject;
            }
        }
        EvalResult::Accept
    }

    fn accept<V: DecisionTreeVisitor>(&mut self, visitor: &mut V) {
        visitor.visit_module(self);
        for child in &mut self.children {
            match child {
                ModuleChildren::PassAllModule(m) => {
                    m.accept(visitor);
                }
                ModuleChildren::SimpleModule(m) => {
                    m.accept(visitor);
                }
                ModuleChildren::Rule(r) => {
                    r.accept(visitor);
                }
            }
        }
        visitor.leave_module(self);
    }
}

pub fn deserialize_module_children(value: &Value) -> ModuleChildren {
    let child_type = value["type"].as_str().unwrap();
    if child_type == "rule" {
        return ModuleChildren::Rule(deserialize_rule(value));
    } else {
        return ModuleChildren::SimpleModule(deserialize_module(value));
    }
}

pub fn deserialize_module(value: &Value) -> SimpleModule {
    let children: Vec<ModuleChildren> = value["children"]
        .as_array()
        .unwrap()
        .into_iter()
        .map(deserialize_module_children)
        .collect();

    let variables: Vec<Variable> = value["variables"]
        .as_array()
        .unwrap()
        .into_iter()
        .map(Variable::deserialize)
        .collect();

    let module = match value["module_type"].as_str().unwrap() {
        "all" => SimpleModule::new(
            value["module_name"].as_str().unwrap().to_string(),
            children,
            Box::from(PassAllModuleStrategy {}),
            variables,
        ),
        "pass_some" => SimpleModule::new(
            value["module_name"].as_str().unwrap().to_string(),
            children,
            Box::from(FailAtMostXModuleStrategy::new(
                value["module_config"].as_object().unwrap()["limit"]
                    .as_i64()
                    .unwrap() as i32,
            )),
            variables,
        ),
        _ => panic!(format!(
            "Unknown module_type: {}",
            value["module_type"].as_str().unwrap()
        )),
    };

    module
}
