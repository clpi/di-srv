use std::{fmt, convert::Infallible};
use div_com::error::DError;
use div_db::sqlx::error::Error as SqlxError;

pub type AResult<T> = Result<T, ApiError>;

#[derive(Debug)]
pub enum ApiError {
    ActixError(actix_web::Error),
    PathError(actix_web::error::PathError),
    DbError(div_com::error::DError),
    SqlxError(div_db::sqlx::Error),
    AwsReqError(div_com::error::DError),
    IoError(div_com::error::DError),
    Inf(Infallible),
}

impl std::error::Error for ApiError {
    fn cause(&self) -> Option<&dyn std::error::Error> {
        match self {
            Self::ActixError(e) => Some(e),
            Self::DbError(DError::DbError(e)) => Some(e),
            Self::SqlxError(sqlx_error) => match sqlx_error {
                SqlxError::Io(e) => Some(e),
                SqlxError::Database(e) => None,
                SqlxError::Decode(e) => None,
                SqlxError::RowNotFound => None,
                SqlxError::PoolTimedOut => None,
                _ => None,
            }
            _ => None,
        }
    }
}


impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ActixError(e) => write!(f, "Actix error: {}", e.to_string()),
            Self::PathError(e) => write!(f, "path error: {}", e),
            Self::SqlxError(e) => write!(f, "Db sqlx error: {}", e),
            Self::DbError(e) => write!(f, "Db error: {}", e),
            Self::AwsReqError(e) => write!(f, "AWS error: {}", e),
            Self::IoError(e) => write!(f, "IO Error: {}", e),
            Self::Inf(e) => write!(f, "Error"),
        }
    }
}


impl From<actix_web::Error> for ApiError {
    fn from(e: actix_web::Error) -> ApiError {
        Self::ActixError(e)
    }
}

impl From<std::io::Error> for ApiError {
    fn from(e: std::io::Error) -> ApiError {
        Self::IoError(DError::IoError(e))
    }
}

impl From<actix_web::error::PathError> for ApiError {
    fn from(e: actix_web::error::PathError) -> ApiError {
        Self::PathError(e)
    }
}

impl From<SqlxError> for ApiError {
    fn from(e: SqlxError) -> Self {
        Self::SqlxError(e)
    }
}



