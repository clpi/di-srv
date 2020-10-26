use divdb::db::*;
use divdb::models::user::User;
use serde::{Serialize, Deserialize};

#[async_std::main]
pub async fn main() -> sqlx::Result<()> {
    let db = divdb::db::Db::new().await?;
    let dbc = db.clone();
    db.init().await?;
    dbc.down().await?.init().await?;
    //async_std::task::block_on(db.down()).unwrap();
    //async_std::task::block_on(dbc.init()).unwrap();
    
    Ok(())

}
