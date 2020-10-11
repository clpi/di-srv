use actix_identity::Identity;
use actix_web::{App,  test, web};
use divapi::app;
use std::{net::TcpListener, sync::mpsc, thread};

pub struct TestApi {
    id: Option<Identity>,
}

pub fn gen_listener() -> std::io::Result<TcpListener> {
    TcpListener::bind("127.0.0.1:0")
}

pub fn gen_open_addr() -> String {
    let listener = gen_listener().expect("Failed to get listener");
    let port = listener.local_addr().unwrap().port();
    format!("http://127.0.0.1:{}", port)
}

pub fn spawn_api() -> std::io::Result<()> {
    let (tx, rx) = mpsc::channel();
    let list = gen_listener().expect("Failed to gen");
    let _ = thread::spawn(move || {
        let _ = app::spawn_api(list, tx).expect("Failed to run");
    });
    let srv = rx.recv().unwrap();
    actix_rt::System::new("").block_on(srv.stop(true));
    Ok(())
}

