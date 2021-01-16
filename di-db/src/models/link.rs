use crate::{db::Db, models::Model};

pub struct Link<'a, F: Model, T: Model> {
    from: Option<&'a F>,
    to: Option<&'a T>,
    from_id: uuid::Uuid,
    to_id: uuid::Uuid,
}

// impl<'a, F, T> Link<'a, F, T> {

    // pub async fn get_from_id<F, T>(db: &Db, id1: uuid::Uuid, id2: uuid::Uuid) -> sqlx::Result<()> {
    //     let from: F = sqlx::query("SELECT * FROM $1 WHERE id = $2")
    //         .bind(F::table())
    //         .bind(id1)
    //         .fetch_one(&db.pool)
    //         .map(|r| F::from_row(r).unwrap())
    //         .await?;
    //     let to: T = sqlx::query("SELECT * FROM $1 WHERE id = $2")
    //         .bind(T::table())
    //         .bind(id2)
    //         .fetch_one(&db.pool)
    //         .map(|r| T::from_row(r).unwrap())
    //         .await?;
    //     Ok(Self { from: Some(&from), to: Some(&to), from_id: from.id, to_id: to.id })
    // }
// }
