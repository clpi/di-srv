-- TODO Handle timezones properly

CREATE TABLE IF NOT EXISTS Users (
    id          UUID NOT NULL PRIMARY KEY,
    email       TEXT NOT NULL UNIQUE,
    username    TEXT NOT NULL UNIQUE CHECK (char_length(username) < 40),
    password    TEXT NOT NULL CHECK (char_length(password) < 40),
    created_at  TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
