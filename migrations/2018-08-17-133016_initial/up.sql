CREATE TABLE decision_strategy (
    decision_strategy_id SERIAL PRIMARY KEY,
    decision_strategy_json JSONB NOT NULL
);

CREATE TABLE decision (
    decision_id SERIAL PRIMARY KEY,
    decision_strategy_id INTEGER REFERENCES decision_strategy NOT NULL,
    application_data JSONB NOT NULL,
    result JSONB
);
