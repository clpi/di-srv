use std::net::Ipv4Addr;
use serde::{Serialize, Deserialize};
use toml::from_str;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    port: u32,
    host: Ipv4Addr,
    pub session_type: SessionType,
    pub jwt_secret: Vec<u8>,
    pub session_key: Option<String>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            port: 7777,
            session_type: SessionType::Redis,
            host: Ipv4Addr::LOCALHOST,
            jwt_secret: Self::jwt_secret().unwrap_or([0;32].to_vec()),
            session_key: Self::session_key(),
        }
    }
}

impl AppConfig {

    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    pub fn session_key() -> Option<String> {
        dotenv::var("SESSION_KEY").ok()
    }

    pub fn database_url() -> Option<String> {
        dotenv::var("DATABASE_URL").ok()
    }

    pub fn jwt_secret() -> Option<Vec<u8>> {
        dotenv::var("JWT_SECRET").ok()
            .map(|k| k.as_bytes().to_vec())
    }

    pub fn from_file() -> Self {
        let c = include_str!("../Config.toml");
        let conf: Self = toml::from_str(c)
            .expect("Could not read TOML file");
        conf
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SessionType {
    Redis,
    Cookie,
    None,
}
