// NOTE: This is the DI server application
// TODO: The following are todo items for d.i.:
//    - [ ] Implement GraphQL endpoint, conditionally compiled for feature = "gql"
//    - [ ] Implement auth flow functionality through Cognito,
//    - [ ] Implement user data fetch / insert for local Postgres db / Dynamo db
//    - [ ] Implement dynamo db crud operations for users, records, items
//    - [ ] Implement graph fxns/data struct (through petgraph?) for record-like relations
//          (i.e. links, relations, attributes -> links, record/item, item/field, etc.)

pub mod app;
pub mod error;
pub mod handlers;
pub mod middleware;
pub mod models;
pub mod state;
pub mod util;
pub mod auth;

// pub mod gql;

use actix_web::*;

pub use handlers::*;
