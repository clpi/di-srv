//use actix_session::{Session, CookieSession};
use std::sync::Mutex;
use actix_cors::Cors;
use actix_session::{CookieSession, Session};
use actix_web::{
    web, App, HttpRequest, HttpResponse, HttpServer, Responder, middleware,
    dev::Server, http::header::{AUTHORIZATION, CONTENT_TYPE, ACCEPT},
};

#[derive(Clone, Copy)]
pub struct Context {

}

impl Context {

    pub fn new() -> Self {
        Self {}
    }

    pub fn lock(self) -> Mutex<Self> {
        Mutex::new(self)
    }
}

#[derive(Clone, Copy)]
pub struct DiCors;

impl DiCors {
    pub fn new() -> Cors {
        let cors = match std::env::var("FRONT_URL").ok() {
            Some(ref url) => Cors::new()
                .allowed_origin(url),
            None => Cors::new()
                .allowed_origin("*")
                .send_wildcard()
        }
            .allowed_headers(vec![CONTENT_TYPE, AUTHORIZATION, ACCEPT])
            .max_age(3600);
        cors
    }
}
