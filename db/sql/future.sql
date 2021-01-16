-- Consider where it might be appropriate to use JSON, i.e.
-- for preferences, user info, etc. also for real/double values
-- learn more about functions, views, search paths, yadda yadda

-- SET statement_timeout = 0;
-- SET lock_timeout = 0;
-- SET idle_in_transaction_session_timeout = 0;
-- SET client_encoding = 'UTF8';
-- SET standard_conforming_strings = on;
-- SET check_function_bodies = false;
-- SET client_min_messages = warning;
-- SET row_security = off;

-- SET search_path = UserData, Objects, Links, Relations;

-- CREATE TYPE status AS ENUM (
    -- 'active',
    -- 'archived',
    -- 'deleted',
    -- 'completed',
-- )

-- CREATE TYPE priority AS ENUM (
    -- 'lowest',
    -- 'low',
    -- 'medium',
    -- 'high',
    -- 'highest',
-- )

-- CREATE TYPE permission AS ENUM (
    -- 'private',
    -- 'invite_only',
    -- 'mutuals_only',
    -- 'public',
-- )

-- CREATE TYPE permission AS ENUM (
    -- 'male',
    -- 'female',
    -- 'other',
-- )

-- CREATE TYPE field_type AS ENUM (
    -- 'dropdown',
    -- 'textbox',
    -- 'enum_select_one',
    -- 'enum_select_mul',
    -- 'boolean',
    -- 'range'
-- )


CREATE SCHEMA IF NOT EXISTS UserData

    CREATE TABLE IF NOT EXISTS UserData.Users (
        id          SERIAL NOT NULL PRIMARY KEY,
        email       TEXT NOT NULL UNIQUE,
        username    TEXT NOT NULL UNIQUE CHECK (char_length(first_name) < 40),
        password    TEXT NOT NULL CHECK (char_length(first_name) < 40),
        created_at  TIMESTAMPZ DEFAULT CURRENT_TIMESTAMP
    )

    CREATE TABLE IF NOT EXISTS UserData.UserInfo (
        id           SERIAL PRIMARY KEY NOT NULL,
        uid          INTEGER NOT NULL REFERENCES UserData.Users(id),
        timezone     TIMEZONE NOT NULL,
        first_name   TEXT CHECK (CHAR_LENGTH(first_name) < 80),
        last_name    TEXT CHECK (CHAR_LENGTH(first_name) < 80),
        bio          TEXT,
        img_path     TEXT,
        gender       TEXT,
        birth_date   INTEGER,
        location     TEXT,
        experience   INTEGER NOT NULL,
        user_type    INTEGER NOT NULL,
        updated_at   TIMESTAMPZ DEFAULT CURRENT_TIMESTAMP
    )

    CREATE TABLE IF NOT EXISTS UserData.Groups (
        id SERIAL PRIMARY KEY NOT NULL,
        name TEXT NOT NULL CHECK (CHAR_LENGTH(name) < 80),
        permission TEXT NOT NULL,
        status TEXT NOT NULL,
        created_at TIMESTAMPZ DEFAULT CURRENT_TIMESTAMP
    )

    CREATE TABLE IF NOT EXISTS UserData.GroupInfo (
        id SERIAL PRIMARY KEY NOT NULL,
        description TEXT NOT NULL,
        private BOOLEAN NOT NULL DEFAULT TRUE,
        status TEXT NOT NULL,
        updated_at TIMESTAMPZ DEFAULT CURRENT_TIMESTAMP
    )

    -- COMMENT ON TABLE UserData.Users is "Essential user info for auth/session"
    -- COMMENT ON TABLE UserData.UserInfo is "Profile info for user"
    -- COMMENT ON TABLE UserData.UserInfo is "Groups of users"

    -- CREATE VIEW UserSession
        -- SELECT (id, email, username) FROM Users

CREATE SCHEMA IF NOT EXISTS Objects

    CREATE TABLE IF NOT EXISTS Objects.Records (
        id SERIAL PRIMARY KEY NOT NULL,
        uid INTEGER NOT NULL REFERENCES UserData.Users(id),
        name TEXT NOT NULL CHECK (CHAR_LENGTH(name) < 80),
        status TEXT NOT NULL,
        private BOOLEAN NOT NULL DEFAULT TRUE,
        created_at TIMESTAMPZ DEFAULT CURRENT_TIMESTAMP
    )

    CREATE TABLE IF NOT EXISTS Objects.Items (
        id SERIAL PRIMARY KEY NOT NULL,
        uid INTEGER NOT NULL REFERENCES UserData.Users(id),
        name TEXT NOT NULL CHECK (CHAR_LENGTH(name) < 80),
        status TEXT NOT NULL,
        private BOOLEAN NOT NULL DEFAULT TRUE,
        created_at TIMESTAMPZ DEFAULT CURRENT_TIMESTAMP
    )


    CREATE TABLE IF NOT EXISTS Objects.Fields (
        id SERIAL PRIMARY KEY NOT NULL,
        name TEXT NOT NULL CHECK (CHAR_LENGTH(name) < 80),
        typ TEXT NOT NULL,
        value TEXT,
        private BOOLEAN NOT NULL DEFAULT TRUE,
        created_at TIMESTAMPZ DEFAULT CURRENT_TIMESTAMP
    )

    CREATE TABLE IF NOT EXISTS Objects.EntryTypes (
        id SERIAL PRIMARY KEY NOT NULL,
        uid INTEGER NOT NULL REFERENCES UserData.Users(id),
        name TEXT NOT NULL CHECK (CHAR_LENGTH(name) < 80),
        private BOOLEAN NOT NULL DEFAULT TRUE,
        created_at TIMESTAMPZ DEFAULT CURRENT_TIMESTAMP
    )

CREATE SCHEMA IF NOT EXISTS Entries

    CREATE TABLE IF NOT EXISTS Entries.EntryEntries ( 
        id SERIAL PRIMARY KEY NOT NULL,
        uid INTEGER NOT NULL REFERENCES UserData.Users(id),
        rid INTEGER NOT NULL REFERENCES Objects.Records(id),
        etid INTEGER REFERENCES Objects.EntryTypes(id),
        created_at TIMESTAMPZ DEFAULT CURRENT_TIMESTAMP
    )

    CREATE TABLE IF NOT EXISTS Entries.FieldEntries ( 
        id SERIAL PRIMARY KEY NOT NULL,
        eeid INTEGER NOT NULL REFERENCES Entries.EntryEntries(id),
        fid INTEGER NOT NULL REFERENCES Objects.Fields(id),
        content TEXT
    )

CREATE SCHEMA IF NOT EXISTS Logic

    CREATE TABLE IF NOT EXISTS Logic.Rules ( 
        id SERIAL PRIMARY KEY NOT NULL,
        uid INTEGER NOT NULL REFERENCES UserData.Users(id),
        name TEXT NOT NULL CHECK (CHAR_LENGTH(name) < 80),
        priority TEXT,
        status TEXT NOT NULL,
        created_at TIMESTAMPZ DEFAULT CURRENT_TIMESTAMP
    )

    CREATE TABLE IF NOT EXISTS Logic.Conditions (
        id SERIAL PRIMARY KEY NOT NULL,
        pos INTEGER NOT NULL,
        and_or BOOLEAN,
        ruleid INTEGER NOT NULL REFERENCES Logic.Rules(id),
        iid1 INTEGER NOT NULL REFERENCES Objects.Items(id),
        iid2 INTEGER NOT NULL REFERENCES Objects.Items(id),
        fid1 INTEGER NOT NULL REFERENCES Objects.Fields(id),
        fid2 INTEGER NOT NULL REFERENCES Objects.Fields(id),
        cond INTEGER NOT NULL,        
        status TEXT NOT NULL,
        created_at TIMESTAMPZ DEFAULT CURRENT_TIMESTAMP
    )

    CREATE TABLE IF NOT EXISTS Logic.Actions (
        id SERIAL PRIMARY KEY NOT NULL,
        ruleid INTEGER NOT NULL REFERENCES Logic.Rules(id),
        target TEXT NOT NULL,
        action TEXT NOT NULL,
        created_at TIMESTAMPZ DEFAULT CURRENT_TIMESTAMP
    )

CREATE SCHEMA IF NOT EXISTS Links

    CREATE TABLE IF NOT EXISTS Links.UserGroupLinks (
        id SERIAL PRIMARY KEY NOT NULL,
        uid INTEGER NOT NULL REFERENCES UserData.Users(id),
        gid INTEGER NOT NULL REFERENCES UserData.Groups(id),
        role TEXT NOT NULL,
        status TEXT NOT NULL,
        created_at TIMESTAMPZ DEFAULT CURRENT_TIMESTAMP
    )

    CREATE TABLE IF NOT EXISTS Links.RecordItemLinks (
        id SERIAL PRIMARY KEY NOT NULL,
        rid INTEGER NOT NULL REFERENCES Objects.Records(id),
        iid INTEGER NOT NULL REFERENCES Objects.Items(id),
        status TEXT NOT NULL,
        priority TEXT,
        created_at TIMESTAMPZ DEFAULT CURRENT_TIMESTAMP
    )

    CREATE TABLE IF NOT EXISTS Links.ItemFieldLinks (
        id SERIAL PRIMARY KEY NOT NULL,
        iid INTEGER NOT NULL REFERENCES Objects.Items(id),
        fid INTEGER NOT NULL REFERENCES Objects.Fields(id),
        priority TEXT,
        created_at TIMESTAMPZ DEFAULT CURRENT_TIMESTAMP
    )

    CREATE TABLE IF NOT EXISTS Links.FieldEntryLinks (
        id SERIAL PRIMARY KEY NOT NULL,
        fid INTEGER NOT NULL REFERENCES Objects.Fields(id),
        etid INTEGER NOT NULL REFERENCES Objects.EntryTypes(id),
        created_at TIMESTAMPZ DEFAULT CURRENT_TIMESTAMP
    )

    CREATE TABLE IF NOT EXISTS Links.UserRecordLinks (
        id SERIAL PRIMARY KEY NOT NULL,
        uid INTEGER NOT NULL REFERENCES UserData.Users(id),
        rid INTEGER NOT NULL REFERENCES Objects.Records(id),
        created_at TIMESTAMPZ DEFAULT CURRENT_TIMESTAMP
    )

CREATE SCHEMA IF NOT EXISTS Relations

    CREATE TABLE IF NOT EXISTS Relations.UserRelations (
        id SERIAL PRIMARY KEY NOT NULL,
        uid1 INTEGER NOT NULL REFERENCES UserData.Users(id),
        uid2 INTEGER NOT NULL REFERENCES UserData.Users(id),
        relation TEXT NOT NULL,
        created_at TIMESTAMPZ DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMPZ DEFAULT CURRENT_TIMESTAMP
    )

    CREATE TABLE IF NOT EXISTS Relations.RecordRelations (
        id SERIAL PRIMARY KEY NOT NULL,
        rid1 INTEGER NOT NULL REFERENCES Objects.Records(id),
        rid2 INTEGER NOT NULL REFERENCES Objects.Records(id),
        relation TEXT NOT NULL,
        created_at TIMESTAMPZ DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMPZ DEFAULT CURRENT_TIMESTAMP
    )

    CREATE TABLE IF NOT EXISTS Relations.ItemRelations (
        id SERIAL PRIMARY KEY NOT NULL,
        iid1 INTEGER NOT NULL REFERENCES Objects.Items(id),
        iid2 INTEGER NOT NULL REFERENCES Objects.Items(id),
        relation TEXT NOT NULL,
        created_at TIMESTAMPZ DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMPZ DEFAULT CURRENT_TIMESTAMP
    )
