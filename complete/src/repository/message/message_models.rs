use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct MessageQueryResult {
    pub id: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub user_id: i64,
    pub body: Option<String>,
    pub image: Option<Vec<u8>>,
    pub likes: i32
}

#[derive(Serialize, FromRow, Clone)]
pub struct MessageWithProfileQueryResult {
    // messsage fields
    pub id: i64,
    pub updated_at: DateTime<Utc>,
    pub body: Option<String>,
    pub likes: i32,
    pub image: Option<Vec<u8>>,  
    // profile fields
    pub user_id: i64,
    pub user_name: String,
    pub full_name: String,
    pub avatar: Option<Vec<u8>>,
    // broadcast message fields
    pub message_broadcast_id: Option<i64>    
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct MessageWithFollowingAndBroadcastQueryResult {
    // messsage fields
    pub id: i64,
    pub updated_at: DateTime<Utc>,
    pub body: Option<String>,
    pub likes: i32,
    pub image: Option<Vec<u8>>,    
    // profile fields
    pub user_id: i64,
    pub user_name: String,
    pub full_name: String,
    pub avatar: Option<Vec<u8>>,
    // broadcast message fields
    pub message_broadcast_id: Option<i64>,
    pub message_broadcast_updated_at: Option<DateTime<Utc>>,
    pub message_broadcast_body: Option<String>,
    pub message_broadcast_likes: Option<i32>,
    pub message_broadcast_image: Option<Vec<u8>>,    
    pub message_broadcast_user_id: Option<i64>,
    pub message_broadcast_user_name: Option<String>,
    pub message_broadcast_full_name: Option<String>,
    pub message_broadcast_avatar: Option<Vec<u8>>
}