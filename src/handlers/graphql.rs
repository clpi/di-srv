use std::io;
use std::sync::Arc;

use actix_cors::Cors;
use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer, Scope};
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;

// use crate::schema::{create_schema, Schema};

// pub fn routes() -> Scope {
//     actix_web::web::scope("/graphql")
//         .service(web::resource("/graphql").route(web::post().to(graphql)))
//         .service(web::resource("/graphiql").route(web::get().to(graphiql)))
// }

// async fn graphiql() -> HttpResponse {
//     let html = graphiql_source("http://127.0.0.1:7777/graphql", None);
//     HttpResponse::Ok()
//         .content_type("text/html; charset=utf-8")
//         .body(html)
// }

// async fn graphql(
//     st: web::Data<Arc<Schema>>,
//     data: web::Json<GraphQLRequest>,
// ) -> Result<HttpResponse, Error> {
//     let user = web::block(move || {
//         let res = data.execute(&st, &());
//         Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
//     })
//     .await?;
//     Ok(HttpResponse::Ok()
//         .content_type("application/json")
//         .body(user))
// }

