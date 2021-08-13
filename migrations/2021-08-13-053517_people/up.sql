-- Your SQL goes here

CREATE TABLE People (
    id INTEGER PRIMARY KEY autoincrement,
    name VARCHAR NOT NULL,
    email VARCHAR NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
)