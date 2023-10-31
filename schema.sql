CREATE TABLE state_changes (
    id TEXT PRIMARY KEY,
    variable_name TEXT NOT NULL,
    old_value TEXT NOT NULL,
    new_value TEXT NOT NULL,
    block_number BIGINT NOT NULL
);



CREATE TABLE variable_tracking (
    id TEXT PRIMARY KEY,
    variable_name TEXT NOT NULL,
    old_value TEXT NOT NULL,
    new_value TEXT NOT NULL,
    block_number BIGINT NOT NULL
);
