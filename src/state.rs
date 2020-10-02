use divdb::db::Db;

pub fn state() -> State {
    let db = Db::new_blocking().unwrap();
    let state = State { db };
    state
}

pub struct Config {
    db_url: String,
    api_key: Vec<u8>,
    secret_key: Vec<u8>,
}

impl Config {
    pub fn new() -> Self {
        Self {
            db_url: String::new(),
            api_key: Vec::new(),
            secret_key: Vec::new(),
        }
    }
}

#[derive(Clone)]
pub struct State {
    pub db: Db,
}

impl State {

}
