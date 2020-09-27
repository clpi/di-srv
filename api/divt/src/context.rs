use divdb::PgPool;
//use com::auth::{get_secret_key, get_jwt_secret};

pub async fn create() -> tide::Result<Context> {
    let db = divdb::Db::new().await.unwrap();
        //.clear().await.unwrap()
        //.init().await.unwrap();

    //let secret_key = get_secret_key().await.unwrap();
    //let jwt_key = get_jwt_secret().await.unwrap();

    let state = Context { 
        data: "Data".to_string(), 
        pool: db.pool.clone(),
        //secret_key, jwt_key
        secret_key: String::new(),
        jwt_key: String::new()
    };
    Ok(state)
}

#[derive(Clone)]
pub struct Context {
    pub data: String,
    pub pool: PgPool,
    pub secret_key: String,
    pub jwt_key: String,
}
