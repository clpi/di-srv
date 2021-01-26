-- use bigserial instead of uuid?
create type if not exists public.status as enum (
    'active', 'archived', 'completed',
    'deleted', 'paused'
)

create type if not exists public.visibility as enum (
    'private',
    'invite_only',
    'mutuals_only',
    'public'
)

create type if not exists public.value_type (
    'integer',
    'decimal',
    'text',
    'date',
    'datetime',
    'duration',
    'boolean',
    'person',
    'place',
    'object',
    'event'
)

CREATE TABLE IF NOT EXISTS public.users (
    id          UUID NOT NULL PRIMARY KEY DEFAULT gen_random_uuid(),
    email       TEXT NOT NULL UNIQUE,
    username    TEXT NOT NULL UNIQUE CHECK (char_length(username) < 40),
    password    TEXT DEFAULT NULL,
    created_at  TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
);

CREATE TABLE IF NOT EXISTS public.user_info (
    id           UUID NOT NULL UNIQUE PRIMARY KEY DEFAULT gen_random_uuid(),
    uid          UUID NOT NULL REFERENCES Users(id),
    first_name   TEXT CHECK (CHAR_LENGTH(first_name) < 80),
    mid_initial  CHAR,
    last_name    TEXT CHECK (CHAR_LENGTH(first_name) < 80),
    phone_number TEXT CHECK (CHAR_LENGTH(phone_number) < 10),
    birth_date   DATE,
    occupation   TEXT,
    bio          TEXT,
    img_path     TEXT,
    gender       TEXT,
    city         TEXT,
    zip_code     TEXT,
    state        TEXT,
    country      TEXT,
    social_links JSON,
    experience   INTEGER  NOT NULL,
    user_type    INTEGER  NOT NULL,
    updated_at   TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
    UNIQUE (uid)
);

CREATE TABLE IF NOT EXISTS public.groups (
    id         UUID NOT NULL PRIMARY KEY DEFAULT gen_random_uuid(),
    uid UUID NOT NULL REFERENCES Users(id),
    name TEXT NOT NULL CHECK (CHAR_LENGTH(name) < 80) UNIQUE,
    description TEXT,
    visibility TEXT,
    status TEXT,
    attributes text[],
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS public.records (
    id         UUID NOT NULL PRIMARY KEY DEFAULT gen_random_uuid(),
    uid UUID NOT NULL REFERENCES Users(id),
    name TEXT NOT NULL CHECK (CHAR_LENGTH(name) < 80),
    description TEXT,
    visibility TEXT,
    status TEXT,
    notes text[],
    attributes text[],
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (name, uid)
);

CREATE TABLE IF NOT EXISTS public.items (
    id         UUID NOT NULL PRIMARY KEY DEFAULT gen_random_uuid(),
    uid UUID NOT NULL REFERENCES Users(id),
    name TEXT NOT NULL CHECK (CHAR_LENGTH(name) < 80),
    description TEXT,
    visibility TEXT,
    status TEXT,
    notes text[],
    attributes text[],
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (name, uid)
);

CREATE TABLE IF NOT EXISTS public.fact_types (
    id UUID NOT NULL PRIMARY KEY DEFAULT gen_random_uuid(),
    uid UUID NOT NULL REFERENCES Users(id),
    name TEXT NOT NULL CHECK (CHAR_LENGTH(name) < 80),
    description TEXT,
    value_type TEXT,
    units TEXT[],
    visibility TEXT,
    status TEXT,
    notes text[],
    attributes text[],
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (uid, name)
);

CREATE TABLE IF NOT EXISTS public.fact_entries (
    id         UUID NOT NULL PRIMARY KEY DEFAULT gen_random_uuid(),
    uid UUID NOT NULL REFERENCES Users(id),
    name TEXT NOT NULL CHECK (CHAR_LENGTH(name) < 80),
    value TEXT NOT NULL,
    units TEXT,
    visibility TEXT,
    notes text[],
    attributes text[],
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (uid, name)
);
