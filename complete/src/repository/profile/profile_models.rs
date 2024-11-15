use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::prelude::FromRow;

#[derive(Serialize, FromRow)]
pub struct ProfileQueryResult {
    pub id: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub user_name: String,
    pub full_name: String,
    pub description: String,
    pub region: Option<String>,
    pub main_url: Option<String>,
    pub avatar: Option<Vec<u8>>,
}

pub struct NewProfile {
    pub user_name: String,
    pub full_name: String,
    pub description: String,
    pub region: Option<String>,
    pub main_url: Option<String>,
    pub avatar: Option<Vec<u8>>,
}