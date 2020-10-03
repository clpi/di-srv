mod common;

use divapi::app;
use std::{net::TcpListener, sync::mpsc, thread, time};

#[actix_rt::test]
async fn index_is_accessible() {}
