extern crate serde;
extern crate serde_json;

use decisionengine::datasource::experian::ApplicationDataV1;
use decisionengine::datasource::experian::ExperianV1_0;
use decisionengine::datasource::experian::ExperianV1_1;
use decisionengine::nodes::EvalNode;
use decisionengine::nodes::NodeResult;

pub mod experian;

pub struct DecisionDataInputNode {
    handler: Box<Fn(&DecisionDataset) -> NodeResult>,
}

impl EvalNode for DecisionDataInputNode {
    fn eval(&mut self, decision_dataset: &DecisionDataset) -> NodeResult {
        (self.handler)(decision_dataset)
    }
}

pub trait DecisionDataRequestHandler<T> {
    fn parse_node(path_parts: &mut Vec<&str>) -> DecisionDataInputNode;
}

#[derive(Serialize, Deserialize)]
pub struct DecisionDataset {
    #[serde(default)]
    application_data_v1: Option<ApplicationDataV1>,

    #[serde(default)]
    experian_v1_0: Option<ExperianV1_0>,

    #[serde(default)]
    experian_v1_1: Option<ExperianV1_1>,
}

impl DecisionDataset {
    pub fn get_empty() -> Self {
        DecisionDataset {
            application_data_v1: None,
            experian_v1_0: None,
            experian_v1_1: None,
        }
    }

    pub fn get_application_data_v1(&self) -> Option<&ApplicationDataV1> {
        self.application_data_v1.as_ref()
    }

    pub fn get_experian_v1_0(&self) -> Option<&ExperianV1_0> {
        self.experian_v1_0.as_ref()
    }

    pub fn get_experian_v1_1(&self) -> Option<&ExperianV1_1> {
        self.experian_v1_1.as_ref()
    }
}

pub fn deserialize_input_node(path: &str) -> (Box<EvalNode>, bool) {
    let mut path_parts: Vec<&str> = path.split(".").collect();
    if path_parts.len() < 2 {
        panic!(format!("Input {} has length < 2", path))
    }
    (
        Box::from(match path_parts.remove(0) {
            "experian_v1_0" => ExperianV1_0::parse_node(&mut path_parts),
            "experian_v1_1" => ExperianV1_1::parse_node(&mut path_parts),
            "application_data_v1" => ApplicationDataV1::parse_node(&mut path_parts),
            _ => panic!(format!("Cannot parse input {}", path)),
        }),
        false,
    )
}
