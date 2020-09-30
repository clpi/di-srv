use divapi::app;
use std::net::TcpListener;

pub fn spawn() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind");
    let port = listener.local_addr().unwrap().port();
    let srv = app::run(listener).expect("Failed to run");
    let _ = tokio::spawn(srv);
    format!("http://127.0.0.1:{}", port)
}

#[actix_rt::test]
async fn index_is_accessible() {
    let res = app::index().await;
    assert!(res.status().is_success())
}
