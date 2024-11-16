use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateMessage {
    pub user_id: i64,
    pub body: String,
    pub broadcasting_msg_id: Option<i64>
}