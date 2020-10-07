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
    birth_date   DATE,
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
    gid INTEGER NOT NULL REFERENCES Groups(id),
    description TEXT NOT NULL,
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
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (name, uid)
);


CREATE TABLE IF NOT EXISTS Fields (
    id SERIAL PRIMARY KEY NOT NULL,
    uid INTEGER NOT NULL REFERENCES Users(id),
    name TEXT NOT NULL CHECK (CHAR_LENGTH(name) < 80),
    field_type TEXT NOT NULL,
    value BYTEA,
    visibility TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (uid, name)
);


CREATE TABLE IF NOT EXISTS ItemEntries ( 
    id SERIAL PRIMARY KEY NOT NULL,
    iid INTEGER NOT NULL REFERENCES Items(id),
    uid INTEGER NOT NULL REFERENCES Users(id),
    rid INTEGER NOT NULL REFERENCES Records(id),
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS FieldEntries (  
    id SERIAL PRIMARY KEY NOT NULL,
    fid INTEGER NOT NULL REFERENCES Fields(id),
    ieid INTEGER NOT NULL REFERENCES ItemEntries(id),
    content BYTEA
);

CREATE TABLE IF NOT EXISTS Rules ( 
    id SERIAL PRIMARY KEY NOT NULL,
    uid INTEGER NOT NULL REFERENCES Users(id),
    name TEXT NOT NULL CHECK (CHAR_LENGTH(name) < 80),
    status TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (uid, name)
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
    rule_id INTEGER NOT NULL REFERENCES Rules(id),
    target TEXT NOT NULL,
    action TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS GroupGroupLinks (
    id SERIAL PRIMARY KEY NOT NULL,
    gid1 INTEGER NOT NULL REFERENCES Groups(id),
    gid2 INTEGER NOT NULL REFERENCES Groups(id),
    status VARCHAR(40) NOT NULL DEFAULT 'active',
    relation VARCHAR(40) NOT NULL DEFAULT 'mutual_of',
    custom BOOLEAN DEFAULT false,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (gid1, gid2, relation)
);

CREATE TABLE IF NOT EXISTS GroupUserLinks (
    id SERIAL PRIMARY KEY NOT NULL,
    gid INTEGER NOT NULL REFERENCES Groups(id),
    uid INTEGER NOT NULL REFERENCES Users(id),
    status VARCHAR(40) NOT NULL DEFAULT 'active',
    relation VARCHAR(40) NOT NULL DEFAULT 'mutual_of',
    custom BOOLEAN DEFAULT false,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (gid, uid, relation)
);

CREATE TABLE IF NOT EXISTS GroupRecordLinks (
    id SERIAL PRIMARY KEY NOT NULL,
    gid INTEGER NOT NULL REFERENCES Groups(id),
    rid INTEGER NOT NULL REFERENCES Records(id),
    status VARCHAR(40) NOT NULL DEFAULT 'active',
    relation VARCHAR(40) NOT NULL DEFAULT 'mutual_of',
    custom BOOLEAN DEFAULT false,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (gid, rid, relation)
);

--------- USER LINKS ---------------------------

CREATE TABLE IF NOT EXISTS UserUserLinks (
    id SERIAL PRIMARY KEY NOT NULL,
    uid1 INTEGER NOT NULL REFERENCES Users(id),
    uid2 INTEGER NOT NULL REFERENCES Users(id),
    status VARCHAR(40) NOT NULL DEFAULT 'active',
    relation VARCHAR(40) NOT NULL DEFAULT 'mutual_of',
    custom BOOLEAN DEFAULT false,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (uid1, uid2, relation)
);

CREATE TABLE IF NOT EXISTS UserRecordLinks (
    id SERIAL PRIMARY KEY NOT NULL,
    uid INTEGER NOT NULL REFERENCES Users(id),
    rid INTEGER NOT NULL REFERENCES Records(id),
    status VARCHAR(40) NOT NULL DEFAULT 'active',
    relation VARCHAR(40) NOT NULL DEFAULT 'mutual_of',
    custom BOOLEAN DEFAULT false,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (uid, rid, relation)
);

CREATE TABLE IF NOT EXISTS UserItemLinks (
    id SERIAL PRIMARY KEY NOT NULL,
    uid INTEGER NOT NULL REFERENCES Users(id),
    iid INTEGER NOT NULL REFERENCES Items(id),
    status VARCHAR(40) NOT NULL DEFAULT 'active',
    relation VARCHAR(40) NOT NULL DEFAULT 'mutual_of',
    custom BOOLEAN DEFAULT false,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (uid, iid, relation)
);

CREATE TABLE IF NOT EXISTS UserRuleLinks (
    id SERIAL PRIMARY KEY NOT NULL,
    uid INTEGER NOT NULL REFERENCES Users(id),
    rule_id INTEGER NOT NULL REFERENCES Rules(id),
    status VARCHAR(40) NOT NULL DEFAULT 'active',
    relation VARCHAR(40) NOT NULL DEFAULT 'mutual_of',
    custom BOOLEAN DEFAULT false,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (uid, rule_id, relation)
);
----------- RECORD LINKS ---------------------------

CREATE TABLE IF NOT EXISTS RecordRecordLinks (
    id SERIAL PRIMARY KEY NOT NULL,
    rid1 INTEGER NOT NULL REFERENCES Records(id),
    rid2 INTEGER NOT NULL REFERENCES Records(id),
    status VARCHAR(40) NOT NULL DEFAULT 'active',
    relation VARCHAR(40) NOT NULL DEFAULT 'mutual_of',
    custom BOOLEAN DEFAULT false,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (rid1, rid2, relation)
);

CREATE TABLE IF NOT EXISTS RecordItemLinks (
    id SERIAL PRIMARY KEY NOT NULL,
    rid INTEGER NOT NULL REFERENCES Records(id),
    iid INTEGER NOT NULL REFERENCES Items(id),
    status VARCHAR(40) NOT NULL DEFAULT 'active',
    relation VARCHAR(40) NOT NULL DEFAULT 'mutual_of',
    custom BOOLEAN DEFAULT false,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (rid, iid, relation)
);

---------- ITEM LINKS -----------------------------

CREATE TABLE IF NOT EXISTS ItemFieldLinks(
    id SERIAL PRIMARY KEY NOT NULL,
    iid INTEGER NOT NULL REFERENCES Items(id),
    fid INTEGER NOT NULL REFERENCES Fields(id),
    status VARCHAR(40) NOT NULL DEFAULT 'active',
    relation VARCHAR(40) NOT NULL DEFAULT 'mutual_of',
    custom BOOLEAN DEFAULT false,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (iid, fid, relation)
);

CREATE TABLE IF NOT EXISTS ItemItemLinks (
    id SERIAL PRIMARY KEY NOT NULL,
    iid1 INTEGER NOT NULL REFERENCES Items(id),
    iid2 INTEGER NOT NULL REFERENCES Items(id),
    status VARCHAR(40) NOT NULL DEFAULT 'active',
    relation VARCHAR(40) NOT NULL DEFAULT 'has_a',
    custom BOOLEAN DEFAULT false,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (iid1, iid2, relation)
);
