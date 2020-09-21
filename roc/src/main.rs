#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

#[get("/")]
pub fn index() -> &'static str {
    "welcome!"
}

#[get("/test")]
pub fn test() -> &'static str {
    "hello, all!"
}

pub fn main() {
    rocket::ignite()
        .mount("/", routes![test, index])
        .launch();
}
