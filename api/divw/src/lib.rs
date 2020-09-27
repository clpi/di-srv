use warp::{Filter, self};
use std::future::Future;
use macros::*;

#[tokio::main]
pub async fn main() {
    route_get!(index, "index", "Hello world".to_string() );

    println!("Hello, world!");
}

pub async fn using<T: Clone + Send>(data: T) -> impl Filter<
        Extract=(T,),
        Error=std::convert::Infallible> + Clone
{
        warp::any().map(move || data.clone())
}

pub trait Model {
    fn table() -> String;
}

#[derive(Clone, Debug)]
pub struct AppData {
    name: &'static str
}

#[derive(Clone)]
pub struct User {
    username: String,
    email: String,
    password: String,
    created_at: i32
}

pub mod macros {

    #[macro_export]
    macro_rules! route_get {
        ($($name:ident, $path:expr, $handler:expr), *) => { (
            pub async fn $name(data: AppData) -> impl Filter<
                Extract=(impl warp::Reply,),
                Error=warp::Rejection> + Clone {
                warp::get()
                    .and(using(data))
                    .and(warp::bodY::json())
                    .and(warp::path!($path))
                    .and_then($handler)
            }
        )+}
    }

    #[macro_export]
    macro_rules! collect_routes {
        ($base:literal, $name:ident, $($route:ident), +) => {
            pub async fn $name(data: AppData) -> impl Filter<
                Extract=(impl warp::Reply,),
                Error=warp::Rejection> + Clone
            {
                warp::path!($base)
                    .and(($route.or($route))+)
            }

        }
    }

}
