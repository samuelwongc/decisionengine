use decisionengine::EvalResult;

trait ResultAggregate {
    fn set_result(&mut self, result: EvalResult);
}

#[derive(Serialize, Deserialize)]
pub struct DecisionResult {
    application_id: u64,
    result: EvalResult,
    module_results: Vec<ModuleResult>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum SubmoduleResult {
    ModuleResult(ModuleResult),
    RuleResult(RuleResult),
}

#[derive(Serialize, Deserialize)]
pub struct ModuleResult {
    pub result: EvalResult,
    pub module_id: String,
    pub submodule_results: Vec<SubmoduleResult>,
}

impl ModuleResult {
    pub fn add_submodule_result(&mut self, result: SubmoduleResult) {
        self.submodule_results.push(result);
    }
}

#[derive(Serialize, Deserialize)]
pub struct RuleResult {
    pub result: EvalResult,
    pub rule_id: i32,
}
