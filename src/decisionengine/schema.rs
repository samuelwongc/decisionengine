table! {
    decision (decision_id) {
        decision_id -> Int4,
        decision_strategy_id -> Int4,
        application_data -> Jsonb,
        result -> Nullable<Jsonb>,
    }
}

table! {
    decision_strategy (decision_strategy_id) {
        decision_strategy_id -> Int4,
        decision_strategy_json -> Jsonb,
    }
}

joinable!(decision -> decision_strategy (decision_strategy_id));

allow_tables_to_appear_in_same_query!(decision, decision_strategy,);
