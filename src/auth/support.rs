pub struct OAuthSupport {}

use std::collections::hash_map::HashMap;
use actix_web::{
    App, dev, web, HttpServer, HttpResponse, Responder,
    middleware::{
        Logger, normalize::{NormalizePath, TrailingSlash,},
    },
};

pub struct OAuthClient {
    // pub client: Client,
    pub secret: Vec<u8>,
}

