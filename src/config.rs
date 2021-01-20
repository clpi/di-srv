use std::net::Ipv4Addr;

#[derive(Debug, Clone)]
pub struct AppConfig {
    port: u32,
    host: Ipv4Addr,
    session: SessionType,
    pub jwt_secret: Vec<u8>,
    pub session_key: Option<String>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            port: 7777,
            session: SessionType::Redis,
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
}

#[derive(Debug, Clone)]
pub enum SessionType {
    Redis,
    Cookie,
    None,
}
