use serde::Serialize;
use sqlx::prelude::FromRow;

pub struct NewProfile {
    pub user_name: String,
    pub full_name: String,
    pub description: String,
    pub region: String,
    pub main_url: String,
    pub avatar: Vec<u8>
}

#[derive(Serialize, FromRow, Clone)]
pub struct Profile {
    pub id: i64,
    pub user_name: String,
    pub full_name: String,
    pub description: String,
    pub region: String,
    pub main_url: String,
    pub avatar: Vec<u8>
}