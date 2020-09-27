
#[derive(PartialEq, Debug, sqlx::Type)]
#[sqlx(rename_all="lowercase")]
pub enum Priority {
    Lowest,
    Low,
    Average,
    High,
    Highest
}

#[derive(PartialEq, Debug, sqlx::Type)]
#[sqlx(rename_all="lowercase")]
pub enum Active {
    Active,
    Deleted,
    Archived,
}

#[derive(PartialEq, Debug, sqlx::Type)]
#[sqlx(rename="record_entry")]
pub struct RecordEntry {
    id: i32,
    rid: i32,
    content: String,
}
