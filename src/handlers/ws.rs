use crate::state::State;
use actix::prelude::*;
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use std::time::{Duration, Instant};

struct Websocket {
    hb: Instant,
}

#[derive(Default)]
struct WsSession {
    id: usize,
    name: Option<String>,
}

impl WsSession {
    pub fn new() {}
}

/*
impl Actor for Websocket {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx)
    }
}

async fn stream(req: HttpRequest, stream: web::Payload, data: web::Data<State>) -> Result<HttpResponse, Error> {
    let res =  ws::start(Websocket::new(), &req, stream);
    println!("Stream: {:?}", res);
    res
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for Websocket {

    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        println!("WS: {:?}", msg);
        match msg {
            Ok(ws::Message::Ping(msg)) => {

            },
            Ok(ws::Message::Pong(_)) => { self.hb = Instant::now() },
            Ok(ws::Message::Text(txt)) => { ctx.text(txt) },
            Ok(ws::Message::Binary(bin)) => { ctx.binary(bin) },
            Ok(ws::Message::Close(rsn)) => {
                ctx.close(rsn);
                ctx.stop();
            },
            _ => ctx.stop()
        }
    }
}


impl Websocket {
    fn new() -> Self {
        Self { hb: Instant::now() }
    }

    /// helper method that sends ping to client every second.
    ///
    /// also this method checks heartbeats from client
    fn hb(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            // check client heartbeats
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                // heartbeat timed out
                println!("Websocket Client heartbeat failed, disconnecting!");

                // stop actor
                ctx.stop();

                // don't try to send a ping
                return;
            }

            ctx.ping(b"");
        });
    }
}

*/
