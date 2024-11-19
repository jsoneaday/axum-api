use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::FromRow;

#[derive(Serialize, FromRow)]
pub struct Follow {
    id: i64,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    follower_id: i64,
    following_id: i64
}