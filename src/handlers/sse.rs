use std::sync::Mutex;
use actix_web::{web::{Bytes, Data}, dev};
use tokio::sync::mpsc::{Sender, Receiver};

pub struct Broadcaster {
    clients: Vec<Sender<Bytes>>,
}

impl Broadcaster {
    /*
    fn create() -> Data<Mutex<Self>> {

    }
    */
}

