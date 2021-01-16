pub struct Config { 
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: String,
    pub database_name: String,
}

impl Config {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        )
    }
}

