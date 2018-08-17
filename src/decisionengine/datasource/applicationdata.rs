use decisionengine::datasource::DecisionDataInputNode;
use decisionengine::datasource::DecisionDataRequestHandler;
use decisionengine::datasource::DecisionDataset;
use decisionengine::nodes::NodeResult;

#[derive(Serialize, Deserialize, Clone)]
pub struct ApplicationDataV1 {
    first_name: String,
    last_name: String,
    age: i32,
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
                "first_name" => Box::from(|decision_dataset: &mut DecisionDataset| {
                    match decision_dataset.get_application_data_v1() {
                        Some(data) => NodeResult::Text(data.first_name()),
                        _ => panic!(format!(
                            "Decision data type {} not included in module but is accessed.",
                            Self::decision_data_type()
                        )),
                    }
                }),
                "last_name" => Box::from(|decision_dataset: &mut DecisionDataset| {
                    match decision_dataset.get_application_data_v1() {
                        Some(data) => NodeResult::Text(data.last_name()),
                        _ => {
                            panic!("Decision data type {} not included in module but is accessed.")
                        }
                    }
                }),
                "age" => {
                    Box::from(|decision_dataset: &mut DecisionDataset| {
                        match decision_dataset.get_application_data_v1() {
                            Some(data) => NodeResult::Numeric(data.age()),
                            _ => panic!(
                                "Decision data type {} not included in module but is accessed."
                            ),
                        }
                    })
                }
                _ => panic!("Unknown query value for Experian v1.1"),
            },
        }
    }
}
