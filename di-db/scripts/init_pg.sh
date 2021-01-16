docker run -itd \
    --restart always \
    -e POSTGRES_USER=casbin_rs \
    -e POSTGRES_PASSWORD=casbin_rs \
    -e POSTGRES_DB=casbin \
    -p 5432:5432 \
    -v /srv/docker/postgresql:/var/lib/postgresql \
    postgres:11;

psql postgres://casbin_rs:casbin_rs@127.0.0.1:5432/casbin -c 
"CREATE TABLE IF NOT EXISTS casbin_rules (
    id SERIAL PRIMARY KEY,
    ptype VARCHAR NOT NULL,
    v0 VARCHAR NOT NULL,
    v1 VARCHAR NOT NULL,
    v2 VARCHAR NOT NULL,
    v3 VARCHAR NOT NULL,
    v4 VARCHAR NOT NULL,
    v5 VARCHAR NOT NULL,
    CONSTRAINT unique_key_sqlx_adapter UNIQUE(ptype, v0, v1, v2, v3, v4, v5)
    );"
