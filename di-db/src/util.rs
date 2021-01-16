
#[macro_export]
macro_rules! impl_get_all {
    ($model:ident, $name:ident) => {
        pub async fn $name(db: &crate::db::Db) -> sqlx::Result<()> {
            Ok(())    
        }
    }
}
