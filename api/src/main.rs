use actix::{Actor, Addr, Arbiter, Context, System, prelude::*};
use actix_web::{
    get, post,
    web::{self, route,}, Responder,
    App, Route, HttpServer, HttpRequest, HttpResponse,
    http::{StatusCode, Cookie,},
};

pub struct InfoBit<'a, T> {
    name: &'static str,
    val: &'a T,
}

#[get("/")]
pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("Welcome to index")
}

#[post("/test")]
pub async fn test(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body.to_uppercase())
}

pub async fn hey() -> impl Responder {
    HttpResponse::Ok().body("hey")
}

//#[actix_rt::main]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(||{
        App::new()
            .service(index)
            .service(test)
            .route("/hey", web::get().to(hey))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
    /*
    let sys = System::new("test");
    let addr = TestActor.start();
    let res = addr.send(TestMsg(3,4)).await;

    match res {
        Ok(result) => println!("OK! {}", result),
        _ => println!("No good"),
    }
    //sys.run();
    */
}

pub struct TestActor;

#[derive(Message)]
#[rtype(result="usize")]
pub struct TestMsg(usize, usize);

impl Actor for TestActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!("Test actor started");
        System::current().stop();
    }
}

impl Handler<TestMsg> for TestActor {
    type Result = usize;

    fn handle(&mut self, msg: TestMsg, ctx: &mut Context<Self>) -> Self::Result {
        msg.0 + msg.1
    }
}
