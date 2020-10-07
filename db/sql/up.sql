
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
    mid_initial  CHAR,
    last_name    TEXT CHECK (CHAR_LENGTH(first_name) < 80),
    phone_number TEXT CHECK (CHAR_LENGTH(phone_number) < 10),
    occupation   TEXT,
    bio          TEXT,
    img_path     TEXT,
    gender       TEXT,
    birth_date   DATE CHECK (birth_date > '1900-01-01'),
    city         TEXT,
    zip_code     TEXT,
    state        TEXT,
    country      TEXT,
    social_links JSON,
    experience   INTEGER NOT NULL,
    user_type    INTEGER NOT NULL,
    updated_at   TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS Groups (
    id SERIAL PRIMARY KEY NOT NULL,
    uid INTEGER NOT NULL REFERENCES Users(id),
    name TEXT NOT NULL CHECK (CHAR_LENGTH(name) < 80) UNIQUE,
    visibility TEXT NOT NULL,
    status TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS GroupInfo (
    id SERIAL PRIMARY KEY NOT NULL,
    description TEXT NOT NULL,
    visibility TEXT NOT NULL,
    status TEXT NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS Records (
    id SERIAL PRIMARY KEY NOT NULL,
    uid INTEGER NOT NULL REFERENCES Users(id),
    name TEXT NOT NULL CHECK (CHAR_LENGTH(name) < 80),
    status TEXT NOT NULL,
    visibility TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (name, uid)
);

CREATE TABLE IF NOT EXISTS Items (
    id SERIAL PRIMARY KEY NOT NULL,
    uid INTEGER NOT NULL REFERENCES Users(id),
    name TEXT NOT NULL CHECK (CHAR_LENGTH(name) < 80),
    status TEXT NOT NULL,
    visibility TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);


CREATE TABLE IF NOT EXISTS Fields (
    id SERIAL PRIMARY KEY NOT NULL,
    uid INTEGER NOT NULL REFERENCES Users(id),
    name TEXT NOT NULL CHECK (CHAR_LENGTH(name) < 80),
    field_type TEXT NOT NULL,
    value BYTEA,
    visibility TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);


CREATE TABLE IF NOT EXISTS ItemEntries ( 
    id SERIAL PRIMARY KEY NOT NULL,
    uid INTEGER NOT NULL REFERENCES Users(id),
    rid INTEGER NOT NULL REFERENCES Records(id),
    iid INTEGER REFERENCES Items(id),
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS FieldEntries (  
    id SERIAL PRIMARY KEY NOT NULL,
    iid INTEGER NOT NULL REFERENCES Items(id),
    fid INTEGER NOT NULL REFERENCES Fields(id),
    content BYTEA,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS Rules ( 
    id SERIAL PRIMARY KEY NOT NULL,
    uid INTEGER NOT NULL REFERENCES Users(id),
    name TEXT NOT NULL CHECK (CHAR_LENGTH(name) < 80),
    priority INTEGER NOT NULL DEFAULT 0,
    status TEXT NOT NULL,
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
    status TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS Actions (
    id SERIAL PRIMARY KEY NOT NULL,
    ruleid INTEGER NOT NULL REFERENCES Rules(id),
    target TEXT NOT NULL,
    action TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Each link only has id, and two ids for each -- then create
-- schema for attributes/relations which map to each link 
-- table entry, such
-- that a bunch of attributes (group role, status, etc.) can be
-- mapped to each link entry, but each link entry can be uniform 

-- LINKS -------------------------------------------

-------- GROUP ------------------------------------

CREATE TABLE IF NOT EXISTS GroupGroupLinks (
    id SERIAL PRIMARY KEY NOT NULL,
    uid INTEGER NOT NULL REFERENCES Users(id),
    gid INTEGER NOT NULL REFERENCES Groups(id),
    status TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS GroupUserLinks (
    id SERIAL PRIMARY KEY NOT NULL,
    uid INTEGER NOT NULL REFERENCES Users(id),
    gid INTEGER NOT NULL REFERENCES Groups(id),
    rid
    status TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS GroupRecordLinks (
    id SERIAL PRIMARY KEY NOT NULL,
    rid INTEGER NOT NULL REFERENCES Records(id),
    iid INTEGER NOT NULL REFERENCES Items(id),
    status TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

--------- USER LINKS ---------------------------

CREATE TABLE IF NOT EXISTS UserUserLinks (
    id SERIAL PRIMARY KEY NOT NULL,
    rid INTEGER NOT NULL REFERENCES Records(id),
    iid INTEGER NOT NULL REFERENCES Items(id),
    status TEXT NOT NULL,
    relation TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS UserRecordLinks (
    id SERIAL PRIMARY KEY NOT NULL,
    rid INTEGER NOT NULL REFERENCES Records(id),
    iid INTEGER NOT NULL REFERENCES Items(id),
    status TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS UserItemLinks (
    id SERIAL PRIMARY KEY NOT NULL,
    rid INTEGER NOT NULL REFERENCES Records(id),
    iid INTEGER NOT NULL REFERENCES Items(id),
    status TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS UserRuleLinks (
    id SERIAL PRIMARY KEY NOT NULL,
    rid INTEGER NOT NULL REFERENCES Records(id),
    iid INTEGER NOT NULL REFERENCES Items(id),
    status TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
----------- RECORD LINKS ---------------------------

CREATE TABLE IF NOT EXISTS RecordRecordLinks (
    id SERIAL PRIMARY KEY NOT NULL,
    rid INTEGER NOT NULL REFERENCES Records(id),
    iid INTEGER NOT NULL REFERENCES Items(id),
    relation TEXT NOT NULL DEFAULT "HasA",
    status TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS RecordItemLinks (
    id SERIAL PRIMARY KEY NOT NULL,
    rid INTEGER NOT NULL REFERENCES Records(id),
    iid INTEGER NOT NULL REFERENCES Items(id),
    relation TEXT NOT NULL DEFAULT "HasA",
    status TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

---------- ITEM LINKS -----------------------------

CREATE TABLE IF NOT EXISTS ItemFieldLinks(
    id SERIAL PRIMARY KEY NOT NULL,
    iid INTEGER NOT NULL REFERENCES Items(id),
    fid INTEGER NOT NULL REFERENCES Fields(id),
    relation TEXT NOT NULL DEFAULT "HasA",
    status TEXT NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (iid, fid, relation)
);

CREATE TABLE IF NOT EXISTS ItemItemLinks (
    id SERIAL PRIMARY KEY NOT NULL,
    iid2 INTEGER NOT NULL REFERENCES Items(id),
    iid1 INTEGER NOT NULL REFERENCES Items(id),
    relation TEXT NOT NULL DEFAULT "HasA",
    status TEXT NOT NULL,
    relation TEXT NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
    UNIQUE (iid, fid, relation)
);

CREATE TABLE IF NOT EXISTS UserRelationLinks (
    id SERIAL PRIMARY KEY NOT NULL,
    iid INTEGER NOT NULL REFERENCES Items(id),
    fid INTEGER NOT NULL REFERENCES Fields(id),
    relation TEXT NOT NULL DEFAULT "HasA",
    status TEXT NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
    UNIQUE (iid, fid, relation)
);
------------- RELATIONS LINK ------------------

-------------- NOTE USER DEFINED ----------------


/*
CREATE TABLE IF NOT EXISTS CustomRelations (
    id,
    table1 TEXT,
    table2 TEXT,
    name TEXT,
)
CREATE TABLE IF NOT EXISTS ItemAttributes (
    id SERIAL PRIMARY KEY NOT NULL,
    uid INTEGER NOT NULL REFERENCES Users(id),
    iid INTEGER NOT NULL REFERENCES Records(id),
    target TEXT NOT NULL,
    key TEXT NOT NULL,
    value TEXT NOT NULL,
)

CREATE TABLE IF NOT EXISTS FieldAttributes (
    id SERIAL PRIMARY KEY NOT NULL,
    uid INTEGER NOT NULL REFERENCES Users(id),
    iid INTEGER NOT NULL REFERENCES Records(id),
    target TEXT NOT NULL,
    key TEXT NOT NULL,
    value TEXT NOT NULL,
)

CREATE TABLE IF NOT EXISTS RecordAttributes (
    id SERIAL PRIMARY KEY NOT NULL,
    uid INTEGER NOT NULL REFERENCES Users(id),
)

CREATE TABLE IF NOT EXISTS GroupAttributes (
    id SERIAL PRIMARY KEY NOT NULL,
    uid INTEGER NOT NULL REFERENCES Users(id),
)

CREATE TABLE IF NOT EXISTS UserRecordItemRelationss (
    id SERIAL PRIMARY KEY NOT NULL,
    uid INTEGER NOT NULL REFERENCES Users(id),
    iid INTEGER NOT NULL REFERENCES Records(id),
    target TEXT NOT NULL,
    key TEXT NOT NULL,
    value TEXT NOT NULL,
)

CREATE TABLE IF NOT EXISTS UserRecordRecordRelations (
    id SERIAL PRIMARY KEY NOT NULL,
    uid INTEGER NOT NULL REFERENCES Users(id),
    iid INTEGER NOT NULL REFERENCES Records(id),
    target TEXT NOT NULL,
    key TEXT NOT NULL,
    value TEXT NOT NULL,
)

CREATE TABLE IF NOT EXISTS UserRecordRecordRelations (
    id SERIAL PRIMARY KEY NOT NULL,
    uid INTEGER NOT NULL REFERENCES Users(id),
    iid INTEGER NOT NULL REFERENCES Records(id),
    target TEXT NOT NULL,
    key TEXT NOT NULL,
    value TEXT NOT NULL,
)

CREATE TABLE IF NOT EXISTS ItemAttributes (
    id SERIAL PRIMARY KEY NOT NULL,
    uid INTEGER NOT NULL REFERENCES Users(id),
    iid INTEGER NOT NULL REFERENCES Records(id),
    target TEXT NOT NULL,
    key TEXT NOT NULL,
    value TEXT NOT NULL,
)
*/

CREATE VIEW UserRecords AS
SELECT r.id, r.uid, r.name, r.status, r.visibility, r.created_at
FROM Records r, Users u
WHERE r.uid = u.id;
