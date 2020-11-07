use url::Url;
use actix::{Context, Actor, Addr, Arbiter, Handler};
use serde::{Serialize, Deserialize};
use std::{
    fmt::Debug,
    collections::HashMap,
    sync::{Arc, RwLock}, io::Read,
};
use actix_web::{
    App, dev, web, HttpServer, HttpResponse, Responder,
    HttpRequest,
    middleware::{
        Logger, normalize::{NormalizePath, TrailingSlash,},
    },
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

#[derive(Debug)]
pub enum AuthType {
    AuthGet,
    AuthPost(String),
    Nothing,
}


pub struct OAuthClient {

}

pub struct OAuthState {
    endpoint: Generic<
        ClientMap,
        AuthMap<RandomGenerator>,
        TokenMap<RandomGenerator>,
        Vacant,
        Vec<Scope>,
        fn() -> OAuthResponse,
    >,
}

impl OAuthState {

    pub fn create() -> Self {
        Self {
            endpoint: Generic {
                registrar: vec![Client::public(
                    "LocalClient",
                    "http://localhost:8021/endpoint"
                        .parse::<Url>()
                        .unwrap()
                        .into(),
                    "default-scope".parse().unwrap(),
                )]
                .into_iter()
                .collect(),
                authorizer: AuthMap::new(RandomGenerator::new(16)),
                issuer: TokenMap::new(RandomGenerator::new(16)),
                solicitor: Vacant,
                scopes: vec!["default-scope".parse().unwrap()],
                response: OAuthResponse::ok,
            },
        }
    }

    pub fn with_solicitor<'a, S>(
        &'a mut self, solicitor: S,
    ) -> impl Endpoint<OAuthRequest, Error = WebError> + 'a
    where
        S: OwnerSolicitor<OAuthRequest> + 'a {
            ErrorInto::new(Generic {
                authorizer: &mut self.endpoint.authorizer,
                registrar: &mut self.endpoint.registrar,
                issuer: &mut self.endpoint.issuer,
                solicitor,
                scopes: &mut self.endpoint.scopes,
                response: OAuthResponse::ok,
            })
    }
}

impl Actor for OAuthState {
    type Context = Context<Self>;
}

impl<Op> Handler<OAuthMessage<Op, AuthType>> for OAuthState
where Op: OAuthOperation {
    type Result = Result<Op::Item, Op::Error>;

    fn handle(&mut self, msg: OAuthMessage<Op, AuthType>, ctx: &mut Self::Context) -> Self::Result {
        let (op, ex) = msg.into_inner();
        match ex {
            AuthType::AuthGet => {
                let solicitor = FnSolicitor(move |_: &mut OAuthRequest, pre_grant: Solicitation| {
                    OwnerConsent::InProgress(
                        OAuthResponse::ok()
                            .content_type("text/html")
                            .expect("Could not get response")
                            .body("<html><body><h1>Hi</h1></body></html>")
                    )
                });
                op.run(self.with_solicitor(solicitor))
            },
            AuthType::AuthPost(query) => {
                let solicitor = FnSolicitor(move |_: &mut OAuthRequest, _: Solicitation| {
                    if query.contains("allow") {
                        OwnerConsent::Authorized("User info".into())
                    } else {
                        OwnerConsent::Denied
                    }
                });
                op.run(self.with_solicitor(solicitor))
            },
            _ => op.run(&mut self.endpoint),
        }
    }
}


pub async fn refresh_token() {}

#[derive(Debug)]
pub enum OAuthType {
    AuthPost(String),
    AuthGet,
    Nothing,
}

pub async fn get_token(
    (req, state): (OAuthRequest, web::Data<Addr<OAuthState>>)
) -> Result<OAuthResponse, WebError>
{
    state.send(Token(req).wrap(AuthType::Nothing)).await?
}

pub async fn get_auth(
    (req, state): (OAuthRequest, web::Data<Addr<OAuthState>>)
) -> Result<OAuthResponse, WebError> {
    state.send(Authorize(req).wrap(AuthType::AuthGet)).await?

}

pub async fn post_auth(
    (http_req, oauth_req, state):
    (HttpRequest, OAuthRequest, web::Data<Addr<OAuthState>>)
) -> Result<OAuthResponse, WebError> {
    state
        .send(Authorize(oauth_req)
            .wrap(AuthType::AuthPost(http_req
                    .query_string()
                    .to_owned()
                    )
                )
            )
        .await?
}

impl OAuthClient {
    pub fn test_server() -> () {
        //let client = Client::get("http://localhost:8888/");
    }
}

pub struct OAuthProvider {

}

pub async fn resource(req: OAuthResource) {}

#[cfg(test)]
pub mod test {

}

