use decisionengine::datasource::DecisionDataInputNode;
use decisionengine::datasource::DecisionDataRequestHandler;
use decisionengine::datasource::DecisionDataset;
use decisionengine::nodes::NodeResult;

#[derive(Serialize, Deserialize)]
pub struct ExperianV1_0 {
    score: i32,
}

#[derive(Serialize, Deserialize)]
pub struct ExperianV1_1 {
    score: i32,
    debt: i32,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationDataV1 {
    first_name: String,
    last_name: String,
    age: i32,
}

impl ExperianV1_0 {
    pub fn score(&self) -> i32 {
        self.score
    }
}

impl DecisionDataRequestHandler<ExperianV1_0> for ExperianV1_0 {
    fn parse_node(path: &mut Vec<&str>) -> DecisionDataInputNode {
        DecisionDataInputNode {
            handler: match path.remove(0) {
                "score" => Box::from(|decision_dataset: &DecisionDataset| {
                    match decision_dataset.get_experian_v1_0() {
                        Some(ref data) => NodeResult::Numeric(data.score()),
                        _ => {
                            panic!("Decision data type {} not included in module but is accessed.")
                        }
                    }
                }),
                _ => panic!("Unknown query value for Experian v1.0"),
            },
        }
    }
}

impl ExperianV1_1 {
    pub fn decision_data_type() -> &'static str {
        "Experian V1.1"
    }

    pub fn score(&self) -> i32 {
        self.score
    }

    pub fn debt(&self) -> i32 {
        self.debt
    }
}

impl DecisionDataRequestHandler<ExperianV1_1> for ExperianV1_1 {
    fn parse_node(path: &mut Vec<&str>) -> DecisionDataInputNode {
        DecisionDataInputNode {
            handler: match path.remove(0) {
                "score" => Box::from(|decision_dataset: &DecisionDataset| {
                    match decision_dataset.get_experian_v1_1() {
                        Some(data) => NodeResult::Numeric(data.score()),
                        _ => panic!(format!(
                            "Decision data type {} not included in module but is accessed.",
                            Self::decision_data_type()
                        )),
                    }
                }),
                "debt" => Box::from(|decision_dataset: &DecisionDataset| {
                    match decision_dataset.get_experian_v1_1() {
                        Some(data) => NodeResult::Numeric(data.debt()),
                        _ => {
                            panic!("Decision data type {} not included in module but is accessed.")
                        }
                    }
                }),
                _ => panic!("Unknown query value for Experian v1.1"),
            },
        }
    }
}

impl ApplicationDataV1 {
    pub fn decision_data_type() -> &'static str {
        "ApplicationData"
    }

    pub fn first_name(&self) -> String {
        self.first_name.clone()
    }

    pub fn last_name(&self) -> String {
        self.last_name.clone()
    }

    pub fn age(&self) -> i32 {
        self.age
    }
}

impl DecisionDataRequestHandler<ApplicationDataV1> for ApplicationDataV1 {
    fn parse_node(path: &mut Vec<&str>) -> DecisionDataInputNode {
        DecisionDataInputNode {
            handler: match path.remove(0) {
                "first_name" => Box::from(|decision_dataset: &DecisionDataset| {
                    match decision_dataset.get_application_data_v1() {
                        Some(data) => NodeResult::Text(data.first_name()),
                        _ => panic!(format!(
                            "Decision data type {} not included in module but is accessed.",
                            Self::decision_data_type()
                        )),
                    }
                }),
                "last_name" => {
                    Box::from(|decision_dataset: &DecisionDataset| {
                        match decision_dataset.get_application_data_v1() {
                            Some(data) => NodeResult::Text(data.last_name()),
                            _ => panic!(
                                "Decision data type {} not included in module but is accessed."
                            ),
                        }
                    })
                }
                "age" => Box::from(|decision_dataset: &DecisionDataset| {
                    match decision_dataset.get_application_data_v1() {
                        Some(data) => NodeResult::Numeric(data.age()),
                        _ => {
                            panic!("Decision data type {} not included in module but is accessed.")
                        }
                    }
                }),
                _ => panic!("Unknown query value for Experian v1.1"),
            },
        }
    }
}
