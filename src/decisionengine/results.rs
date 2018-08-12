use decisionengine::datasource::DecisionDataset;
use decisionengine::rules::Condition;
use decisionengine::rules::Rule;
use decisionengine::EvalResult;
use decisionengine::Evaluatable;

trait ResultAggregate {
    fn set_result(&mut self, result: EvalResult);
}

struct DecisionResult {
    application_id: u64,
    result: EvalResult,
    module_results: Vec<ModuleResult>,
}

enum SubmoduleResult {
    ModuleResult(ModuleResult),
    RuleResult(RuleResult),
}

struct ModuleResult {
    result: EvalResult,
    module_id: String,
    submodule_results: Vec<SubmoduleResult>,
}

struct RuleResult {
    result: EvalResult,
    rule_id: String,
}

struct ResultAggregator {
    result: ResultAggregate,
}

impl ResultAggregator {
    pub fn set_result(&mut self, eval_result: EvalResult) {
        self.result.set_result(eval_result)
    }
}

struct ResultAggregatingDecorator<'a, T>
where
    T: Evaluatable,
{
    t: T,
    result: &'a mut ResultAggregate,
}

impl<'a, T> Evaluatable for ResultAggregatingDecorator<'a, T>
where
    T: Evaluatable,
{
    fn eval(&mut self, input: &DecisionDataset) -> EvalResult {
        let eval_result = self.t.eval(input);
        self.result.set_result(eval_result.clone());
        eval_result
    }
}
