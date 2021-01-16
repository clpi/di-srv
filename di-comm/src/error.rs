use std::fmt::{Formatter, Result as FResult};

#[derive(Debug)]
pub enum DError {
    DbError(sqlx::Error),
    EnvError(dotenv::Error),
    AttError(dynomite::AttributeError),
    IoError(std::io::Error),
}

pub type DResult<T> = Result<T, DError>;

impl std::fmt::Display for DError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        match self {
            Self::DbError(e) => write!(f, "DB ERROR: {}", e),
            Self::EnvError(e) => write!(f, "ENV ERROR: {}", e),
            Self::AttError(e) => write!(f, "ATT ERROR: {}", e),
            Self::IoError(e) => write!(f, "IO ERROR: {}", e),
        }
    }
}

impl std::error::Error for DError {

    fn cause(&self) -> Option<&dyn std::error::Error> {
        match self {
            Self::DbError(e) => Some(e),
            Self::EnvError(e) => Some(e),
            Self::AttError(e) => Some(e),
            Self::IoError(e) => Some(e)
        }
    }
}

impl From<sqlx::Error> for DError {
    fn from(e: sqlx::Error) -> Self {
        Self::DbError(e)
    }
}

impl From<dotenv::Error> for DError {
    fn from(e: dotenv::Error) -> Self {
        Self::EnvError(e)
    }
}

impl From<dynomite::AttributeError> for DError {
    fn from(e: dynomite::AttributeError) -> Self {
        Self::AttError(e)
    }
}

impl From<std::io::Error> for DError {
    fn from(e: std::io::Error) -> Self {
        Self::IoError(e)
    }
}


