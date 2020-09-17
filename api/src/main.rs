use actix::{Actor, Addr, Arbiter, Context, System, prelude::*};

#[actix_rt::main]
async fn main() {
    let sys = System::new("test");
    let addr = TestActor.start();
    let res = addr.send(TestMsg(3,4)).await;

    match res {
        Ok(result) => println!("OK! {}", result),
        _ => println!("No good"),
    }
    //sys.run();
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
