#[derive(Clone)]
pub struct Auth {
    key: String,
}

impl Auth {

    pub fn new() -> Self {
        Self { key: std::env::var("SECRET_KEY").unwrap_or_else(|_| "0018".repeat(8)) }
    }

    pub fn hash<T: Into<String>>(self, pass: T) -> Result<String, ArgonError> {
        Ok(pass.into())
    }

    pub fn verify<T: Into<String>, U: Into<String>>(self, pass: T, hash: U) -> Result<bool, ArgonError> {
        Ok(pass.into())
    }

}

