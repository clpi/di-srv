use actix_web::{middleware::Condition, web, dev::{ServiceRequest, ServiceResponse}, error::ErrorUnauthorized, Error, HttpResponse, http};
use actix_identity::RequestIdentity;
use actix_service::{Transform, Service};
use crate::state::State;
use futures::{Future, future::{ok, Either, Ready}, future::PollFn};
use std::{pin::Pin, boxed::Box, task::{Poll, Context}};

pub struct Auth;
impl<S, B> Transform<S> for Auth
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static, B: 'static,
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
pub struct AuthMiddleware<S> { service: S }
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
            Some(_id) => { Either::Left(self.service.call(req)) },
            None => { 
                match req.path() {
                    "/auth/login" => Either::Left(self.service.call(req)),
                    _ => Either::Right(ok(req.into_response(
                        HttpResponse::Found()
                            .header(http::header::LOCATION, "/auth/login")
                            .finish().into_body()
                    )))
                }
            },
        }
    }

    fn poll_ready(&mut self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)   
    }
}

