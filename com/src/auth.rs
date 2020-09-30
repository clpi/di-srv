use argonautica::{
    Hasher, Verifier, Error as ArgonError,
};

#[derive(Clone)]
pub struct Auth {
    key: String,
}

impl Auth {

    pub fn new() -> Self {
        Self { key: std::env::var("SECRET_KEY").unwrap_or_else(|_| "0018".repeat(8)) }
    }

    pub fn hash<T: Into<String>>(self, pass: T) -> Result<String, ArgonError> {
        let res = Hasher::default()
            .with_password(pass.into())
            .with_secret_key(self.key.as_str())
            .hash()?;
        Ok(res)
    }

    pub fn verify<T: Into<String>, U: Into<String>>(self, pass: T, hash: U) -> Result<bool, ArgonError> {
        let res = Verifier::default()
            .with_hash(hash.into())
            .with_password(pass.into())
            .with_secret_key(self.key.as_str())
            .verify()?;
        Ok(res)
    }

}

