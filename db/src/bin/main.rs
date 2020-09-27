use divdb::db::*;
use divdb::models::user::User;
use serde::{Serialize, Deserialize};

#[async_std::main]
pub async fn main() -> sqlx::Result<()> {
    let db = divdb::db::Db::new().await?;
    let users = get_all_users(&db.pool).await?;
    //add_user(&db.pool, User::new("c@div.is", "c", "p")).await?;
    users.into_iter().for_each(|user| {
        println!("User: {}", user.username);
    });
    Ok(())

}
