use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use uuid::Uuid;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Claim {
    pub uid: Uuid,
    pub email: String,
    exp: i64,
}

impl Claim {
    pub fn new(uid: Uuid, email: String) -> Self {
        Self {
            uid,
            email,
            exp: (Utc::now() + Duration::hours(72)).timestamp(),
        }
    }
}


pub fn new_jwt(c: Claim) -> jsonwebtoken::errors::Result<String> {
    let encoding_key = EncodingKey::from_secret(dotenv::var("JWT_SECRET").unwrap().as_str().as_bytes());
    encode(&Header::default(),&c,&encoding_key)
}

pub fn decode_jwt(token: &str) -> jsonwebtoken::errors::Result<Claim> {
    let sec = dotenv::var("JWT_SECRET").unwrap();
    let sec = sec.as_str().as_bytes();
    let decoding_key = DecodingKey::from_secret(&sec);
    decode::<Claim>(token, &decoding_key, &Validation::default())
        .map(|data| data.claims)
}
