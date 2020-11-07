pub mod support;
pub mod oauth;
pub mod jwt;
use actix_web::{
    web::{
        Payload, PayloadConfig, get
    },
    http::HeaderName,
};
use serde::{Serialize, Deserialize};
use url::Url;
use actix::{
    Actor,  Context, Handler,
};
use actix_web::middleware::{
        Logger,
        normalize::{NormalizePath, TrailingSlash}
    };
use oxide_auth::{
    primitives::prelude::{
        AuthMap, Client, ClientMap,
        RandomGenerator, Scope, TokenMap
    },
    endpoint::{
        Endpoint, OwnerConsent, OwnerSolicitor,
        Solicitation,
    },
    frontends::simple::endpoint::{
        ErrorInto, FnSolicitor, Generic, Vacant
    },
};
use oxide_auth_actix::{
    Authorize, OAuthMessage, OAuthOperation,
    OAuthRequest, OAuthResource, OAuthResponse,
    Refresh, Resource, Token, WebError,
};

#[derive(Debug, Default)]
pub(crate) struct State {
    pub token: Option<String>,
    pub refresh: Option<String>,
    pub until: Option<i64>,
}

impl State {

    pub fn new(
        token: Option<String>, refresh: Option<String>, until: Option<i64>) -> Self { Self { token, refresh, until }
    }


    pub async fn test() {
        let _web = web::to("dfd");
    }
}

pub struct Request {
    id: Uuid,
    val: String,
}
