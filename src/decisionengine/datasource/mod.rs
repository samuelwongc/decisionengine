extern crate serde;
extern crate serde_json;

use decisionengine::datasource::applicationdata::ApplicationDataV1;
use decisionengine::datasource::experian::ExperianV1_0;
use decisionengine::datasource::experian::ExperianV1_1;
use decisionengine::datasource::mocks::decisiondatafetcher::{MockedExperianV1_0Fetcher,
                                                             MockedExperianV1_1Fetcher};
use decisionengine::nodes::EvalNode;
use decisionengine::nodes::NodeResult;

pub mod applicationdata;
pub mod experian;
pub mod mocks;

pub struct DecisionDataInputNode {
    handler: Box<Fn(&mut DecisionDataset) -> NodeResult>,
}

impl EvalNode for DecisionDataInputNode {
    fn eval(&mut self, decision_dataset: &mut DecisionDataset) -> NodeResult {
        (self.handler)(decision_dataset)
    }
}

pub trait DecisionDataRequestHandler<T> {
    fn parse_node(path_parts: &mut Vec<&str>) -> DecisionDataInputNode;
}

pub struct DecisionDataset {
    application_data_v1: Option<ApplicationDataV1>,

    experian_v1_0: Option<ExperianV1_0>,
    experian_v1_0_fetcher: Box<DecisionDataFetcher<ApplicationDataV1, ExperianV1_0>>,

    experian_v1_1: Option<ExperianV1_1>,
    experian_v1_1_fetcher: Box<DecisionDataFetcher<ApplicationDataV1, ExperianV1_1>>,
}

trait DecisionDataFetcher<D, R> {
    fn fetch(&self, data: &D) -> R;
}

impl DecisionDataset {
    pub fn new(application_data: ApplicationDataV1) -> Self {
        DecisionDataset {
            application_data_v1: Some(application_data),
            experian_v1_0: None,
            experian_v1_0_fetcher: Box::from(MockedExperianV1_0Fetcher::test()),
            experian_v1_1: None,
            experian_v1_1_fetcher: Box::from(MockedExperianV1_1Fetcher::test()),
        }
    }

    pub fn get_empty() -> Self {
        DecisionDataset {
            application_data_v1: None,
            experian_v1_0: None,
            experian_v1_0_fetcher: Box::from(MockedExperianV1_0Fetcher::test()),
            experian_v1_1: None,
            experian_v1_1_fetcher: Box::from(MockedExperianV1_1Fetcher::test()),
        }
    }

    pub fn get_application_data_v1(&self) -> Option<&ApplicationDataV1> {
        self.application_data_v1.as_ref()
    }

    pub fn get_experian_v1_0(&mut self) -> Option<&ExperianV1_0> {
        if self.experian_v1_0.is_none() {
            self.experian_v1_0 = Some(
                self.experian_v1_0_fetcher
                    .fetch(self.application_data_v1.as_ref().unwrap()),
            )
        }
        self.experian_v1_0.as_ref()
    }

    pub fn get_experian_v1_1(&mut self) -> Option<&ExperianV1_1> {
        if self.experian_v1_1.is_none() {
            self.experian_v1_1 = Some(
                self.experian_v1_1_fetcher
                    .fetch(self.application_data_v1.as_ref().unwrap()),
            )
        }
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
