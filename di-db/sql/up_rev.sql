-- TODO Handle timezones properly
-- TODO actually use this enums as types (how do they play with sqlx?)

-- CREATE TYPE status AS ENUM (
    -- 'active',
    -- 'archived',
    -- 'deleted',
    -- 'completed'
-- );

-- CREATE TYPE priority AS ENUM (
    -- 'lowest',
    -- 'low',
    -- 'medium',
    -- 'high',
    -- 'highest'
-- );

-- CREATE TYPE permission AS ENUM (
    -- 'private',
    -- 'invite_only',
    -- 'mutuals_only',
    -- 'public'
-- );

-- CREATE TYPE permission AS ENUM (
    -- 'male',
    -- 'female',
    -- 'other'
-- );

-- CREATE TYPE field_type AS ENUM (
    -- 'dropdown',
    -- 'textbox',
    -- 'enum_select_one',
    -- 'enum_select_mul',
    -- 'boolean',
    -- 'range'
-- );

CREATE TABLE IF NOT EXISTS Users (
    id          SERIAL NOT NULL PRIMARY KEY,
    email       TEXT NOT NULL UNIQUE,
    username    TEXT NOT NULL UNIQUE CHECK (char_length(username) < 40),
    password    TEXT NOT NULL,
    created_at  TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS UserInfo (
    id           SERIAL PRIMARY KEY NOT NULL,
    uid          INTEGER NOT NULL REFERENCES Users(id),
    first_name   TEXT CHECK (CHAR_LENGTH(first_name) < 80),
    last_name    TEXT CHECK (CHAR_LENGTH(first_name) < 80),
    bio          TEXT,
    img_path     TEXT,
    gender       TEXT,
    birth_date   INTEGER,
    location     TEXT,
    experience   INTEGER NOT NULL,
    user_type    INTEGER NOT NULL,
    updated_at   TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS Groups (
    id SERIAL PRIMARY KEY NOT NULL,
    name TEXT NOT NULL CHECK (CHAR_LENGTH(name) < 80),
    permission TEXT NOT NULL,
    status INTEGER NOT NULL DEFAULT 1,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS GroupInfo (
    id SERIAL PRIMARY KEY NOT NULL,
    description TEXT NOT NULL,
    visibility INTEGER NOT NULL DEFAULT 0,
    status INTEGER NOT NULL DEFAULT 1,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS Records (
    id SERIAL PRIMARY KEY NOT NULL,
    uid INTEGER NOT NULL REFERENCES Users(id),
    name TEXT NOT NULL CHECK (CHAR_LENGTH(name) < 80),
    status INTEGER NOT NULL DEFAULT 1,
    visibility INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS Items (
    id SERIAL PRIMARY KEY NOT NULL,
    uid INTEGER NOT NULL REFERENCES Users(id),
    name TEXT NOT NULL CHECK (CHAR_LENGTH(name) < 80),
    status INTEGER NOT NULL DEFAULT 1,
    visibility INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);


CREATE TABLE IF NOT EXISTS Fields (
    id SERIAL PRIMARY KEY NOT NULL,
    name TEXT NOT NULL CHECK (CHAR_LENGTH(name) < 80),
    typ TEXT NOT NULL,
    value TEXT,
    visibility INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS EntryTypes (
    id SERIAL PRIMARY KEY NOT NULL,
    uid INTEGER NOT NULL REFERENCES Users(id),
    name TEXT NOT NULL CHECK (CHAR_LENGTH(name) < 80),
    visibility INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS EntryEntries( 
    id SERIAL PRIMARY KEY NOT NULL,
    uid INTEGER NOT NULL REFERENCES Users(id),
    rid INTEGER NOT NULL REFERENCES Records(id),
    etid INTEGER REFERENCES EntryTypes(id),
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS FieldEntries( 
    id SERIAL PRIMARY KEY NOT NULL,
    eeid INTEGER NOT NULL REFERENCES EntryTypes(id),
    fid INTEGER NOT NULL REFERENCES Fields(id),
    content TEXT
);

CREATE TABLE IF NOT EXISTS Rules ( 
    id SERIAL PRIMARY KEY NOT NULL,
    uid INTEGER NOT NULL REFERENCES Users(id),
    name TEXT NOT NULL CHECK (CHAR_LENGTH(name) < 80),
    priority INTEGER NOT NULL DEFAULT 0,
    status INTEGER NOT NULL DEFAULT 1,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS Conditions (
    id SERIAL PRIMARY KEY NOT NULL,
    pos INTEGER NOT NULL,
    and_or BOOLEAN,
    ruleid INTEGER NOT NULL REFERENCES Rules(id),
    iid1 INTEGER NOT NULL REFERENCES Items(id),
    iid2 INTEGER NOT NULL REFERENCES Items(id),
    fid1 INTEGER NOT NULL REFERENCES Fields(id),
    fid2 INTEGER NOT NULL REFERENCES Fields(id),
    cond INTEGER NOT NULL,        
    status INTEGER NOT NULL DEFAULT 1,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS Actions (
    id SERIAL PRIMARY KEY NOT NULL,
    ruleid INTEGER NOT NULL REFERENCES Rules(id),
    target TEXT NOT NULL,
    action TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS UserGroupLinks(
    id SERIAL PRIMARY KEY NOT NULL,
    uid INTEGER NOT NULL REFERENCES Users(id),
    gid INTEGER NOT NULL REFERENCES Groups(id),
    role TEXT NOT NULL,
    status INTEGER NOT NULL DEFAULT 1,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS RecordItemLinks(
    id SERIAL PRIMARY KEY NOT NULL,
    rid INTEGER NOT NULL REFERENCES Records(id),
    iid INTEGER NOT NULL REFERENCES Items(id),
    status INTEGER NOT NULL DEFAULT 1,
    priority INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS ItemFieldLinks(
    id SERIAL PRIMARY KEY NOT NULL,
    iid INTEGER NOT NULL REFERENCES Items(id),
    fid INTEGER NOT NULL REFERENCES Fields(id),
     INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS FieldEntryLinks(
    id SERIAL PRIMARY KEY NOT NULL,
    fid INTEGER NOT NULL REFERENCES Fields(id),
    etid INTEGER NOT NULL REFERENCES EntryTypes(id),
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS UserRecordLinks(
    id SERIAL PRIMARY KEY NOT NULL,
    uid INTEGER NOT NULL REFERENCES Users(id),
    rid INTEGER NOT NULL REFERENCES Records(id),
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS UserRelations(
    id SERIAL PRIMARY KEY NOT NULL,
    uid1 INTEGER NOT NULL REFERENCES Users(id),
    uid2 INTEGER NOT NULL REFERENCES Users(id),
    relation TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS RecordRelations(
    id SERIAL PRIMARY KEY NOT NULL,
    rid1 INTEGER NOT NULL REFERENCES Records(id),
    rid2 INTEGER NOT NULL REFERENCES Records(id),
    relation TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS ItemRelations(
    id SERIAL PRIMARY KEY NOT NULL,
    iid1 INTEGER NOT NULL REFERENCES Items(id),
    iid2 INTEGER NOT NULL REFERENCES Items(id),
    relation TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
