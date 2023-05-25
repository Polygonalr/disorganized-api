use diesel::prelude::*;

#[derive(Queryable)]
pub struct ViewToken {
    pub id: i32,
    pub token: String,
    pub filepath: String,
    pub expiry_date: Option<chrono::NaiveDateTime>,
    pub created_at: chrono::NaiveDateTime,
}