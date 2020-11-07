pub struct OAuthSupport {}

use std::collections::hash_map::HashMap;
use oxide_auth::primitives::prelude::{
    Client, ClientUrl, Scope, Issuer, IssuedToken,
    AuthMap, Authorizer, PreGrant, TokenMap, TokenSigner,
    Registrar, Assertion
};

use actix_web::{
    App, dev, web, HttpServer, HttpResponse, Responder,
    middleware::{
        Logger, normalize::{NormalizePath, TrailingSlash,},
    },
};

pub struct OAuthClient {
    pub client: Client,
    pub secret: Vec<u8>,
}

impl Default for OAuthClient {
    fn default() -> Self {
        Self {
            client: Client::public(
                "DummyClient",
                "https://localhost:7777/"
                    .parse::<url::Url>()
                    .expect("Could not parse to URL")
                    .into(),
                "test-scope".parse()
                    .expect("Could not parse scope"),
            ),
            secret: Vec::new(),
        }
    }
}

impl OAuthClient {

    pub async fn new() -> Self {
        let client = Client::public(
            "DummyClient",
            "http://localhost:7777/"
                .parse::<url::Url>()
                .expect("Could not parse to URL")
                .into(),
            "test-scope".parse()
                .expect("Could not parse scope"),
        );
        Self::default()
    }
}
