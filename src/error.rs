use std::{fmt, convert::Infallible};
use div_com::error::DError;

#[derive(Debug)]
pub enum ApiError {
    ActixError(actix_web::Error),
    PathError(actix_web::error::PathError),
    DbError(div_com::error::DError),
    AwsReqError(div_com::error::DError),
    IoError(div_com::error::DError)
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ActixError(e) => write!(f, "Actix error: {}", e.to_string()),
            Self::PathError(e) => write!(f, "path error: {}", e),
            Self::DbError(e) => write!(f, "Db error: {}", e),
            Self::AwsReqError(e) => write!(f, "AWS error: {}", e),
            Self::IoError(e) => write!(f, "IO Error: {}", e)
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



#[derive(Debug)]
enum AuthError {
    Invalid,
    DoesNotExist,
    Internal,
}

#[derive(Debug)]
pub enum OAuthError {
    AccessFailed,
    NoToken,
    AuthorizationFailed,
    RefreshFailed,
    Invalid(serde_json::Error),
    MissingToken,
    Response(String),
}

