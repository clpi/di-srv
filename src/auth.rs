use tokio::*;
use serde::{Serialize, Deserialize};
use div_cloud::cognito::*;
use derive_more::*;
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
    auth_url: AuthUrl,
    client_id: ClientId,
    client_secret: ClientSecret,
    resource: Resource,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Req(OAuthRequest);

impl OAuthReq {
    pub fn new() -> Self {

    }
}

pub struct ApiError {}

pub enum ApiErrorKind {

}

