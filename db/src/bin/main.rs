use divdb::db::*;
use divdb::models::user::User;
use serde::{Serialize, Deserialize};

#[async_std::main]
pub async fn main() -> sqlx::Result<()> {
    let db = divdb::db::Db::new().await?;
    //db.clone().down().await?;
    db.init().await?;

    /*
    let u = User {
        email: "duh@div.is".into(), 
        username: "duh".into(), 
        password:"p".into(), ..User::default()
    };
    async_std::task::block_on(u.insert(&db)).unwrap();
    //add_user(&db.pool, User::new("c@div.is", "c", "p")).await?;
    let users = get_all_users(&db.pool).await?;
    users.into_iter().for_each(|user| {
        println!("User: {}", user.username);
    });
    */
    Ok(())

}
