cargo install --version=0.1.0-beta.1 sqlx-cli --no-default-features --features postgres
export DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@localhost:${DB_PORT}/${DB_NAME}
sqlx database create
sqlx migrate add create_users_table
