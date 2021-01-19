pub mod support;
pub mod jwt;

#[derive(Debug, Default)]
pub(crate) struct State {
    pub token: Option<String>,
    pub refresh: Option<String>,
    pub until: Option<i64>,
}

pub struct PwVerifier<'p> {
    config: argon2::Config<'p>,
}

impl<'p> PwVerifier<'p> {
    pub fn new() -> Self {
        let cf = argon2::Config {
            variant: argon2::Variant::Argon2i,
            version: argon2::Version::Version13,
            ad: Self::secret().unwrap_or(&[]), //NOTE as env var for prod
            secret: Self::secret().unwrap_or(&[]), //NOTE as env var for prod,
            time_cost: 10,
            mem_cost: 65536,
            lanes: 4,
            hash_length: 32,
            thread_mode: argon2::ThreadMode::Parallel,
        };
        Self { config: cf }
    }

    pub fn secret<'a>() -> Result<&'a[u8], dotenv::Error> {
        let se = match option_env!("HASH_SECRET_KEY") {
            Some(ev) => ev,
            None => dotenv::var("HASH_SECRET_KEY")?.as_str(),
        };
        Ok(se.as_bytes())
    }

    pub fn ad<'a>() -> Result<&'a[u8], dotenv::Error> {
        let se = match option_env!("AD") {
            Some(ev) => ev,
            None => dotenv::var("AD")?.as_str(),
        };
        Ok(se.as_bytes())
    }

    pub fn hash(&self, pw: &str) -> argon2::Result<String> {
        let salt = &[];
        let hash = argon2::hash_encoded(pw.as_bytes(), salt, &self.config)?;
        return Ok(hash);
    }

    pub fn verify(&self, pw: &str, hash: &str) -> argon2::Result<bool> {
        argon2::verify_encoded_ext(
            &hash,
            &pw.as_bytes(),
            Self::secret().expect("HASH_SECET_KEY not set"),
            Self::ad().expect("AD not set"),
        )
    }

}

impl State {

    pub fn _new(
        token: Option<String>, refresh: Option<String>, until: Option<i64>) -> Self { Self { token, refresh, until }
    }
}

