use decisionengine::datasource::DecisionDataset;
use decisionengine::modules::PassAllModule;
use decisionengine::results::ModuleResult;
use decisionengine::results::RuleResult;
use decisionengine::results::SubmoduleResult;
use decisionengine::rules::Condition;
use decisionengine::rules::Rule;
use decisionengine::EvalResult;
use decisionengine::Evaluatable;

use std;

pub trait DecisionTreeVisitor {
    fn visit_pass_all_module(&mut self, module: &mut PassAllModule);
    fn visit_rule(&mut self, rule: &mut Rule);
    fn leave_pass_all_module(&mut self, module: &mut PassAllModule);
    fn leave_rule(&mut self, rule: &mut Rule);
    fn visit_condition(&mut self, condition: &Condition);
}

pub struct ResultAggregatingVisitor {
    pub stack: ResultStack,
    pub input: DecisionDataset,
}

pub struct ResultStack {
    curr: Option<Box<ResultStackElement>>,
}

struct ResultStackElement {
    value: SubmoduleResult,
    prev: Option<Box<ResultStackElement>>,
}

impl ResultStackElement {
    pub fn value(&mut self) -> &mut SubmoduleResult {
        &mut self.value
    }
}

impl ResultStack {
    pub fn get_result(&self) -> &SubmoduleResult {
        &self.curr.as_ref().unwrap().value
    }

    pub fn new(init: SubmoduleResult) -> Self {
        ResultStack {
            curr: Some(Box::from(ResultStackElement {
                value: init,
                prev: None,
            })),
        }
    }

    pub fn new_module(&mut self, module_id: String, result: EvalResult) {
        let mut top = ResultStackElement {
            value: SubmoduleResult::ModuleResult(ModuleResult {
                module_id: module_id,
                result: result,
                submodule_results: Vec::new(),
            }),
            prev: None,
        };
        std::mem::swap(&mut top.prev, &mut self.curr);
        self.curr = Some(Box::from(top));
    }

    pub fn end_module(&mut self) {
        if self.curr.is_none() {
            panic!("End module");
        }

        let curr = std::mem::replace(&mut self.curr, None);

        let ResultStackElement { mut prev, value } = *curr.unwrap();

        match prev.as_mut().unwrap().value() {
            SubmoduleResult::ModuleResult(ref mut res) => {
                res.add_submodule_result(value);
            }
            _ => {}
        }

        std::mem::replace(&mut self.curr, prev);
    }

    pub fn last_mut(&mut self) -> &mut SubmoduleResult {
        &mut self.curr.as_mut().unwrap().value
    }
}

impl DecisionTreeVisitor for ResultAggregatingVisitor {
    fn visit_pass_all_module(&mut self, module: &mut PassAllModule) {
        self.stack
            .new_module(module.module_name.clone(), module.eval(&mut self.input));
    }

    fn leave_pass_all_module(&mut self, _module: &mut PassAllModule) {
        self.stack.end_module();
    }

    fn visit_rule(&mut self, rule: &mut Rule) {
        match self.stack.last_mut() {
            SubmoduleResult::ModuleResult(ref mut res) => {
                res.add_submodule_result(SubmoduleResult::RuleResult(RuleResult {
                    rule_id: rule.rule_id,
                    result: rule.eval(&mut self.input),
                }));
            }
            _ => panic!("Something went wrong during visiting rule"),
        }
    }

    fn leave_rule(&mut self, _rule: &mut Rule) {}

    fn visit_condition(&mut self, _condition: &Condition) {
        unimplemented!();
    }
}
