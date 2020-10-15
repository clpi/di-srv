use crate::state::State;
use actix_identity::RequestIdentity;
use actix_service::{Service, Transform};
use actix_web::{
    dev::{ServiceRequest, ServiceResponse},
    error::ErrorUnauthorized,
    http,
    middleware::Condition,
    web, Error, HttpResponse,
};
use futures::{
    future::PollFn,
    future::{ok, Either, Ready},
    Future,
};
use std::{
    boxed::Box,
    pin::Pin,
    task::{Context, Poll},
};
use actix_web_httpauth::extractors::basic::BasicAuth;

pub(crate) async fn validator_fn(req: ServiceRequest, cred: BasicAuth,) 
    -> Result<ServiceRequest, Error> 
{
    Ok(req)
}

pub(crate) struct Auth;
impl<S, B> Transform<S> for Auth
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddleware { service })
    }
}

pub(crate) struct AuthMiddleware<S> {
    service: S,
}

impl<S, B> Service for AuthMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Either<S::Future, Ready<Result<Self::Response, Self::Error>>>;

    fn call(&mut self, req: Self::Request) -> Self::Future {
        println!("Req: {:?}", req.path());
        match RequestIdentity::get_identity(&req) {
            Some(_id) => Either::Left(self.service.call(req)),
            None => match req.path() {
                "/auth/login" => Either::Left(self.service.call(req)),
                _ => Either::Right(ok(req.into_response(
                    HttpResponse::Found()
                        .header(http::header::LOCATION, "/auth/login")
                        .finish()
                        .into_body(),
                ))),
            },
        }
    }

    fn poll_ready(&mut self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }
}
