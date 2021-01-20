pub mod jwt;

use crate::config::AppConfig;
use jsonwebtoken::{
    Validation, encode, decode, Header, DecodingKey, EncodingKey, TokenData,
};
use div_db::models::User;
use serde::{Serialize, Deserialize};
use actix_web_httpauth::{
    extractors::{AuthenticationError, bearer::{BearerAuth, Config}},
    middleware::HttpAuthentication,
};

#[derive(Debug, Default)]
pub(crate) struct State {
    pub token: Option<String>,
    pub refresh: Option<String>,
    pub until: Option<i64>,
}

pub struct PwVerifier {
}

impl PwVerifier {
    pub fn new() -> Self {
        let _cf = argon2::Config {
            variant: argon2::Variant::Argon2i,
            version: argon2::Version::Version13,
            secret: Self::sk().expect("HASH_SECRET_KEY not set").as_bytes(),
            ad: Self::ad().expect("AD NOT SET").as_bytes(),
            time_cost: 10,
            mem_cost: 65536,
            lanes: 4,
            hash_length: 32,
            thread_mode: argon2::ThreadMode::Parallel,
        };
        Self {  }
    }

    pub fn ad() -> Result<String, dotenv::Error> {
        match option_env!("HASH_SECRET_KEY") {
            Some(ev) => Ok(ev.to_string()),
            None => Ok(dotenv::var("HASH_SECRET_KEY")?),
        }
    }

    pub fn sk() -> Result<String, dotenv::Error> {
        match option_env!("HASH_SECRET_KEY") {
            Some(ev) => Ok(ev.to_string()),
            None => Ok(dotenv::var("HASH_SECRET_KEY")?),
        }
    }

    pub fn hash(&self, pw: &str) -> argon2::Result<String> {
        let sk = Self::sk().expect("HASH_SECRET_KEY not set");
        let ad = Self::ad().expect("AD not set");
        let cf = argon2::Config {
            variant: argon2::Variant::Argon2i,
            version: argon2::Version::Version13,
            secret: sk.as_bytes(),
            ad: ad.as_bytes(),
            time_cost: 10,
            mem_cost: 65536,
            lanes: 4,
            hash_length: 32,
            thread_mode: argon2::ThreadMode::Parallel,
        };
        let salt = &[];
        let hash = argon2::hash_encoded(pw.as_bytes(), salt, &cf)?;
        return Ok(hash);
    }

    pub fn verify(&self, pw: &str, hash: &str) -> argon2::Result<bool> {
        argon2::verify_encoded_ext(
            &hash,
            &pw.as_bytes(),
            Self::sk().unwrap().as_bytes(),
            Self::ad().unwrap().as_bytes(),
        )
    }

}

impl State {

    pub fn _new(
        token: Option<String>, refresh: Option<String>, until: Option<i64>) -> Self { Self { token, refresh, until }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub uid: uuid::Uuid,
    pub email: String,
    pub exp: i32,
}


impl Claims {
    pub fn decode(tok: &str) -> jsonwebtoken::errors::Result<TokenData<Claims>> {
        let v = Validation {
            validate_exp: false,
            ..Default::default()
        };
        decode::<Claims>(&tok,
            &DecodingKey::from_secret(AppConfig::jwt_secret().unwrap_or([0_u8; 32].to_vec()).as_ref()),
            &v)

    }
    pub fn encode(user: User) -> jsonwebtoken::errors::Result<String> {
        let cl = Claims {
            uid: user.id,
            email: user.username,
            exp: 3600,
        };
        encode(
            &Header::default(),
            &cl,
            &EncodingKey::from_secret(AppConfig::jwt_secret().unwrap_or([0_u8; 32].to_vec()).as_ref())
        )
    }
}


// async fn validator(credentials: BearerAuth) -> Result<ServiceRequest, Error> {
//     match auth::validate_token(credentials.token()) {
//         Ok(res) => {
//             if res == true {
//                 Ok(req)
//             } else {
//                 Err(AuthenticationError::from(config).into())
//             }
//         }
//         Err(_) => Err(AuthenticationError::from(config).into()),
//     }
// }
