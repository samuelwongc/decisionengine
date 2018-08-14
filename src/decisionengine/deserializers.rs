// use decisionengine::modules::PassAllModule;
// use decisionengine::results::ModuleResult;
// use decisionengine::results::ResultAggregatingDecorator;
// use decisionengine::results::SubmoduleResult::RuleResult;
// use decisionengine::rules::{Condition, Rule};
// use decisionengine::Evaluatable;
// use serde_json::Value;
// use std::collections::HashMap;

// trait Deserializer {
//     fn deserialize_rule(&self, value: &Value) -> Box<Evaluatable>;
//     fn deserialize_module(&self, value: &Value) -> Box<Evaluatable>;
// }

// pub struct DefaultDeserializer {}

// impl Deserializer for DefaultDeserializer {
//     fn deserialize_module(&self, value: &Value) -> Box<Evaluatable> {
//         let children = value["children"]
//             .as_array()
//             .unwrap()
//             .into_iter()
//             .map(|x| match x["type"].as_str().unwrap() {
//                 "rule" => Box::from(self.deserialize_rule(x)),
//                 "module" => self.deserialize_module(x),
//                 _ => panic!("Unknown module children type"),
//             })
//             .collect();

//         let module = match value["module_type"].as_str().unwrap() {
//             "all" => PassAllModule {
//                 module_name: value["module_name"].as_str().unwrap().to_string(),
//                 children: children,
//             },
//             _ => panic!(format!(
//                 "Unknown module_type: {}",
//                 value["module_type"].as_str().unwrap()
//             )),
//         };

//         Box::from(module)
//     }

//     fn deserialize_rule(&self, value: &Value) -> Box<Evaluatable> {
//         let mut conditions = HashMap::new();
//         for condition in value["conditions"].as_array().unwrap() {
//             let r = Condition::deserialize(condition);
//             conditions.insert(r.condition_id, r);
//         }

//         Box::from(Rule {
//             rule_name: value["rule_name"].as_str().unwrap().to_string(),
//             rule_id: value["rule_id"].as_i64().unwrap() as i32,
//             conditions: conditions,
//         })
//     }
// }

// // pub struct ResultAggregatingDeserializer {
// //     result: ModuleResult,
// // }

// // impl ResultAggregatingDeserializer {
// //     pub fn result(&self) -> &ModuleResult {
// //         &self.result
// //     }

// //     pub fn set_result(&mut self, result: ModuleResult) {
// //         self.result = result;
// //     }
// // }

// // impl Deserializer for ResultAggregatingDeserializer {
// //     fn deserialize_module(&self, value: &Value) -> Box<Evaluatable> {
// //         let children = value["children"]
// //             .as_array()
// //             .unwrap()
// //             .into_iter()
// //             .map(|x| match x["type"].as_str().unwrap() {
// //                 "rule" => Box::from(self.deserialize_rule(x)),
// //                 "module" => self.deserialize_module(x),
// //                 _ => panic!("Unknown module children type"),
// //             })
// //             .collect();

// //         let module = match value["module_type"].as_str().unwrap() {
// //             "all" => PassAllModule {
// //                 module_name: value["module_name"].as_str().unwrap().to_string(),
// //                 children: children,
// //             },
// //             _ => panic!(format!(
// //                 "Unknown module_type: {}",
// //                 value["module_type"].as_str().unwrap()
// //             )),
// //         };

// //         Box::from(module)
// //     }

// //     fn deserialize_rule(&self, value: &Value) -> Box<Evaluatable> {
// //         let mut conditions = HashMap::new();
// //         for condition in value["conditions"].as_array().unwrap() {
// //             let r = Condition::deserialize(condition);
// //             conditions.insert(r.condition_id, r);
// //         }

// //         Box::from(ResultAggregatingDecorator {
// //             t: Rule {
// //                 rule_name: value["rule_name"].as_str().unwrap().to_string(),
// //                 rule_id: value["rule_id"].as_i64().unwrap() as i32,
// //                 conditions: conditions,
// //             },
// //             result: RuleResult,
// //         })
// //     }
// // }
