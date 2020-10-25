CREATE TABLE IF NOT EXISTS Users (
    id          UUID NOT NULL PRIMARY KEY,
    email       TEXT NOT NULL UNIQUE,
    username    TEXT NOT NULL UNIQUE CHECK (char_length(username) < 40),
    password    TEXT DEFAULT NULL,
    provider    TEXT DEFAULT 'diweb',
    created_at  TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS UserInfo (
    id           UUID PRIMARY KEY NOT NULL,
    uid          UUID NOT NULL REFERENCES Users(id),
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
    experience   INTEGER  NOT NULL,
    user_type    INTEGER  NOT NULL,
    updated_at   TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS Groups (
    id UUID PRIMARY KEY NOT NULL,
    uid UUID NOT NULL REFERENCES Users(id),
    name TEXT NOT NULL CHECK (CHAR_LENGTH(name) < 80) UNIQUE,
    visibility TEXT NOT NULL,
    status TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS GroupInfo (
    id UUID PRIMARY KEY NOT NULL,
    gid UUID NOT NULL REFERENCES Groups(id),
    description TEXT NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS Records (
    id UUID PRIMARY KEY NOT NULL,
    uid UUID NOT NULL REFERENCES Users(id),
    name TEXT NOT NULL CHECK (CHAR_LENGTH(name) < 80),
    status TEXT NOT NULL,
    visibility TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (name, uid)
);

CREATE TABLE IF NOT EXISTS Items (
    id UUID PRIMARY KEY NOT NULL,
    uid UUID NOT NULL REFERENCES Users(id),
    name TEXT NOT NULL CHECK (CHAR_LENGTH(name) < 80),
    status TEXT NOT NULL,
    visibility TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (name, uid)
);


CREATE TABLE IF NOT EXISTS Fields (
    id UUID PRIMARY KEY NOT NULL,
    uid UUID NOT NULL REFERENCES Users(id),
    name TEXT NOT NULL CHECK (CHAR_LENGTH(name) < 80),
    field_type TEXT NOT NULL,
    visibility TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (uid, name)
);


CREATE TABLE IF NOT EXISTS ItemEntries ( 
    id UUID PRIMARY KEY NOT NULL,
    iid UUID NOT NULL REFERENCES Items(id),
    uid UUID NOT NULL REFERENCES Users(id),
    rid UUID NOT NULL REFERENCES Records(id),
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS FieldEntries (  
    id UUID PRIMARY KEY NOT NULL,
    fid UUID NOT NULL REFERENCES Fields(id),
    ieid UUID NOT NULL REFERENCES ItemEntries(id),
    value BYTEA,
    content BYTEA
);

CREATE TABLE IF NOT EXISTS Rules ( 
    id UUID PRIMARY KEY NOT NULL,
    uid UUID NOT NULL REFERENCES Users(id),
    name TEXT NOT NULL CHECK (CHAR_LENGTH(name) < 80),
    status TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (uid, name)
);

CREATE TABLE IF NOT EXISTS Conditions (
    id UUID PRIMARY KEY NOT NULL,
    pos UUID NOT NULL,
    and_or BOOLEAN,
    ruleid UUID NOT NULL REFERENCES Rules(id),
    iid1 UUID NOT NULL REFERENCES Items(id),
    iid2 UUID NOT NULL REFERENCES Items(id),
    fid1 UUID NOT NULL REFERENCES Fields(id),
    fid2 UUID NOT NULL REFERENCES Fields(id),
    cond UUID NOT NULL,        
    status TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS Actions (
    id UUID PRIMARY KEY NOT NULL,
    rule_id UUID NOT NULL REFERENCES Rules(id),
    target TEXT NOT NULL,
    action TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS GroupGroupLinks (
    id UUID PRIMARY KEY NOT NULL,
    gid1 UUID NOT NULL REFERENCES Groups(id),
    gid2 UUID NOT NULL REFERENCES Groups(id),
    status UUID NOT NULL DEFAULT 'active',
    relation VARCHAR(40) NOT NULL DEFAULT 'mutual_of',
    crid UUID DEFAULT NULL REFERENCES Relations(id),
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (gid1, gid2, relation)
);

CREATE TABLE IF NOT EXISTS GroupUserLinks (
    id UUID PRIMARY KEY NOT NULL,
    gid UUID NOT NULL REFERENCES Groups(id),
    uid UUID NOT NULL REFERENCES Users(id),
    status VARCHAR(40) NOT NULL DEFAULT 'active',
    relation VARCHAR(40) NOT NULL DEFAULT 'mutual_of',
    crid UUID DEFAULT NULL REFERENCES Relations(id),
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (gid, uid, relation)
);

CREATE TABLE IF NOT EXISTS GroupRecordLinks (
    id UUID PRIMARY KEY NOT NULL,
    gid UUID NOT NULL REFERENCES Groups(id),
    rid UUID NOT NULL REFERENCES Records(id),
    status VARCHAR(40) NOT NULL DEFAULT 'active',
    relation VARCHAR(40) NOT NULL DEFAULT 'mutual_of',
    crid UUID DEFAULT NULL REFERENCES Relations(id),
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (gid, rid, relation)
);

------------  RELATION ----------------------------

CREATE TABLE IF NOT EXISTS Relations (
    id UUID PRIMARY KEY NOT NULL,
    uid UUID NOT NULL REFERENCES Users(id),
    key VARCHAR(40) NOT NULL,
    value VARCHAR(40) NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (key, value, uid)
);

------------- TAGS ----------------------------

CREATE TABLE IF NOT EXISTS Tags (
    id UUID PRIMARY KEY NOT NULL,
    uid UUID NOT NULL REFERENCES Users(id),
    key VARCHAR(40) NOT NULL,
    value VARCHAR(40) NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (key, value, uid)
);

CREATE TABLE IF NOT EXISTS ItemTags (
    id UUID PRIMARY KEY NOT NULL,
    tid UUID NOT NULL REFERENCES Tags(id),
    iid UUID NOT NULL REFERENCES Items(id),
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (key, uid, iid)
);

CREATE TABLE IF NOT EXISTS RecordTags (
    id UUID PRIMARY KEY NOT NULL,
    tid UUID NOT NULL REFERENCES Tags(id),
    rid VARCHAR(40) NOT NULL REFERENCES Records(id),
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (key, uid, rid)
);

CREATE TABLE IF NOT EXISTS FieldTags (
    id UUID PRIMARY KEY NOT NULL,
    tid UUID NOT NULL REFERENCES Tags(id),
    fid VARCHAR(40) NOT NULL REFERENCES Fields(id),
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (key, uid, fid)
);

--------- USER LINKS ---------------------------

CREATE TABLE IF NOT EXISTS UserUserLinks (
    id SERIAL PRIMARY KEY NOT NULL,
    uid1 UUID NOT NULL REFERENCES Users(id),
    uid2 UUID NOT NULL REFERENCES Users(id),
    status VARCHAR(40) NOT NULL DEFAULT 'active',
    relation VARCHAR(40) NOT NULL DEFAULT 'mutual_of',
    crid UUID DEFAULT NULL REFERENCES Relations(id),
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (uid1, uid2, relation)
);

CREATE TABLE IF NOT EXISTS UserRecordLinks (
    id SERIAL PRIMARY KEY NOT NULL,
    uid UUID NOT NULL REFERENCES Users(id),
    rid UUID NOT NULL REFERENCES Records(id),
    status VARCHAR(40) NOT NULL DEFAULT 'active',
    relation VARCHAR(40) NOT NULL DEFAULT 'mutual_of',
    crid UUID DEFAULT NULL REFERENCES Relations(id),
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (uid, rid, relation)
);

CREATE TABLE IF NOT EXISTS UserItemLinks (
    id SERIAL PRIMARY KEY NOT NULL,
    uid UUID NOT NULL REFERENCES Users(id),
    iid UUID NOT NULL REFERENCES Items(id),
    status VARCHAR(40) NOT NULL DEFAULT 'active',
    relation VARCHAR(40) NOT NULL DEFAULT 'mutual_of',
    crid UUID DEFAULT NULL REFERENCES Relations(id),
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (uid, iid, relation)
);

CREATE TABLE IF NOT EXISTS UserRuleLinks (
    id SERIAL PRIMARY KEY NOT NULL,
    uid UUID NOT NULL REFERENCES Users(id),
    rule_id UUID NOT NULL REFERENCES Rules(id),
    status VARCHAR(40) NOT NULL DEFAULT 'active',
    relation VARCHAR(40) NOT NULL DEFAULT 'mutual_of',
    crid UUID DEFAULT NULL REFERENCES Relations(id),
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (uid, rule_id, relation)
);
----------- RECORD LINKS ---------------------------

CREATE TABLE IF NOT EXISTS RecordRecordLinks (
    id SERIAL PRIMARY KEY NOT NULL,
    rid1 UUID NOT NULL REFERENCES Records(id),
    rid2 UUID NOT NULL REFERENCES Records(id),
    status VARCHAR(40) NOT NULL DEFAULT 'active',
    relation VARCHAR(40) NOT NULL DEFAULT 'mutual_of',
    crid UUID DEFAULT NULL REFERENCES Relations(id),
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (rid1, rid2, relation)
);

CREATE TABLE IF NOT EXISTS RecordItemLinks (
    id SERIAL PRIMARY KEY NOT NULL,
    rid UUID NOT NULL REFERENCES Records(id),
    iid UUID NOT NULL REFERENCES Items(id),
    status VARCHAR(40) NOT NULL DEFAULT 'active',
    relation VARCHAR(40) NOT NULL DEFAULT 'mutual_of',
    crid UUID DEFAULT NULL REFERENCES Relations(id),
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (rid, iid, relation)
);

---------- ITEM LINKS -----------------------------

CREATE TABLE IF NOT EXISTS ItemFieldLinks(
    id SERIAL PRIMARY KEY NOT NULL,
    iid UUID NOT NULL REFERENCES Items(id),
    fid UUID NOT NULL REFERENCES Fields(id),
    status VARCHAR(40) NOT NULL DEFAULT 'active',
    relation VARCHAR(40) NOT NULL DEFAULT 'mutual_of',
    crid UUID DEFAULT NULL REFERENCES Relations(id),
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (iid, fid, relation)
);

CREATE TABLE IF NOT EXISTS ItemItemLinks (
    id SERIAL PRIMARY KEY NOT NULL,
    iid1 UUID NOT NULL REFERENCES Items(id),
    iid2 UUID NOT NULL REFERENCES Items(id),
    status VARCHAR(40) NOT NULL DEFAULT 'active',
    relation VARCHAR(40) NOT NULL DEFAULT 'has_a',
    crid UUID DEFAULT NULL REFERENCES Relations(id),
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (iid1, iid2, relation)
);

CREATE TABLE IF NOT EXISTS FieldFieldLink (
    id SERIAL PRIMARY KEY NOT NULL,
    fid1 UUID NOT NULL REFERENCES Fields(id),
    fid2 UUID NOT NULL REFERENCES Fields(id),
    status VARCHAR(40) NOT NULL DEFAULT 'active',
    relation VARCHAR(40) NOT NULL DEFAULT 'has_a',
    crid UUID DEFAULT NULL REFERENCES Relations(id),
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (iid1, iid2, relation)
);

CREATE TABLE IF NOT EXISTS Relations (
    id SERIAL PRIMARY KEY NOT NULL,
    uid UUID NOT NULL REFERENCES Users(id),
    name VARCHAR(40) NOT NULL,
    value BYTEA,
    status VARCHAR(40) NOT NULL DEFAULT 'active',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,

)
-- relation rule link
-- rule action link
