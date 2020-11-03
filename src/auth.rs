use serde::{Serialize, Deserialize};
use oxide_auth::{
    code_grant::{
        authorization::{
            Input, Authorization, authorization_code,
            Request, Pending, Output, Extension, Endpoint,
        },
    },
    primitives::prelude::*,
};
use oxide_auth::endpoint::{self, Issuer, Registrar};
use oxide_auth::*;
use oxide_auth_actix::{Refresh, Token, WebError, Resource, Authorize,
    OAuthRequest, OAuthMessage,
    OAuthResponse,
};

#[derive(Default, Debug, PartialEq, PartialOrd, Serialize, Deserialize, Clone)]
pub struct OAuthReq {
    auth_url: String,
    client_id: String,
    client_secret: String,
    resource: String,
}

impl OAuthReq {
    pub fn new() -> Self {
        Self { ..Default::default() }
    }
}

pub struct ApiError {}

pub enum ApiErrorKind {

}

