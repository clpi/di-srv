CREATE TABLE IF NOT EXISTS UserInfo (
    id           UUID PRIMARY KEY NOT NULL,
    uid          UUID NOT NULL REFERENCES Users(id),
    first_name   TEXT CHECK (CHAR_LENGTH(first_name) < 80),
    last_name    TEXT CHECK (CHAR_LENGTH(first_name) < 80),
    bio          TEXT,
    img_path     TEXT,
    gender       TEXT,
    birth_date   INTEGER,
    location     TEXT,
    experience   INTEGER NOT NULL,
    user_type    INTEGER NOT NULL,
    updated_at   TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
