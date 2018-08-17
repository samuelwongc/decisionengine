use decisionengine::datasource::applicationdata::ApplicationDataV1;
use decisionengine::datasource::experian::{ExperianV1_0, ExperianV1_1};
use decisionengine::datasource::DecisionDataFetcher;

pub struct MockedExperianV1_1Fetcher {
    data: ExperianV1_1,
}

impl MockedExperianV1_1Fetcher {
    pub fn new(data: ExperianV1_1) -> Self {
        Self { data: data }
    }

    pub fn test() -> Self {
        Self {
            data: ExperianV1_1 {
                score: 1000,
                debt: 0,
            },
        }
    }
}

impl DecisionDataFetcher<ApplicationDataV1, ExperianV1_1> for MockedExperianV1_1Fetcher {
    fn fetch(&self, _a: &ApplicationDataV1) -> ExperianV1_1 {
        self.data.clone()
    }
}

pub struct MockedExperianV1_0Fetcher {
    data: ExperianV1_0,
}

impl MockedExperianV1_0Fetcher {
    pub fn new(data: ExperianV1_0) -> Self {
        Self { data: data }
    }

    pub fn test() -> Self {
        Self {
            data: ExperianV1_0 { score: 1000 },
        }
    }
}

impl DecisionDataFetcher<ApplicationDataV1, ExperianV1_0> for MockedExperianV1_0Fetcher {
    fn fetch(&self, _a: &ApplicationDataV1) -> ExperianV1_0 {
        self.data.clone()
    }
}
