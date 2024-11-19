use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{Error, PgPool};
use sqlx::query_as;
use crate::repository::repo::{DbRepo, EntityId};
use tracing::error;
use super::message_models::{MessageWithFollowingAndBroadcastQueryResult, MessageWithProfileQueryResult};

#[async_trait]
pub trait MessageRepo {
    async fn insert_message(&self, pool: &PgPool, user_id: i64, body: &str, broadcasting_msg_id: Option<i64>) -> Result<EntityId, Error>;    
    async fn insert_response_message(
        conn: &PgPool,
        user_id: i64,
        body: &str,
        original_msg_id: i64
    ) -> Result<i64, sqlx::Error>;
    async fn select_message(&self, pool: &PgPool, id: i64) -> Result<Option<MessageWithFollowingAndBroadcastQueryResult>, Error>;
    async fn select_messages(
        conn: &PgPool,
        user_id: i64,
        last_updated_at: DateTime<Utc>,
        page_size: i16
    ) -> Result<Vec<MessageWithFollowingAndBroadcastQueryResult>, sqlx::Error>;
}

#[async_trait]
impl MessageRepo for DbRepo {
    async fn insert_message(&self, pool: &PgPool, user_id: i64, body: &str, broadcasting_msg_id: Option<i64>) -> Result<EntityId, Error> {
        let mut tx = pool.begin().await.unwrap();

        let insert_msg_result = query_as::<_, EntityId>(
                "insert into message (user_id, body) values ($1, $2) returning id"
            )
            .bind(user_id)
            .bind(body)
            .fetch_one(&mut *tx)
            .await;

        let message_id = match insert_msg_result {
            Ok(entity) => entity.id,
            Err(e) => {
                error!("insert_message error: {}", e);
                return Err(e);
            }
        };

        if let Some(bm_id) = broadcasting_msg_id {
            let message_broadcast_result = query_as::<_, EntityId>(
                    "insert into message_broadcast (main_msg_id, broadcasting_msg_id) values ($1, $2) returning id"
                )
                .bind(message_id)
                .bind(bm_id)
                .fetch_one(&mut *tx).await;

            if message_broadcast_result.is_err() {
                _ = tx.rollback().await;
                return Err(message_broadcast_result.err().unwrap());
            }
        }

        _ = tx.commit().await;

        Ok(EntityId { id: message_id })
    }

    async fn insert_response_message(
        conn: &PgPool,
        user_id: i64,
        body: &str,
        original_msg_id: i64
    ) -> Result<i64, sqlx::Error> {
        let mut tx = conn.begin().await.unwrap();

        match query_as::<_, EntityId>(
                "insert into message (user_id, body) values ($1, $2) returning id"
            )
            .bind(user_id)
            .bind(body)
            .fetch_one(&mut *tx)
            .await {
                Ok(msg_entity) => {
                    match query_as::<_, EntityId>(
                        "insert into message_response (original_msg_id, responding_msg_id) values ($1, $2) returning id"
                    )
                    .bind(original_msg_id)
                    .bind(msg_entity.id)
                    .fetch_one(&mut *tx)
                    .await {                        
                        Ok(_) => {
                            _ = tx.commit().await;
                            Ok(msg_entity.id)
                        },
                        Err(e) => {
                            error!("insert_response_message failed: {}", e);
                            _ = tx.rollback().await;    
                            Err(e)
                        },
                    }     
                },
                Err(e) => {
                    error!("insert_response_message failed: {}", e);
                    _ = tx.rollback().await;
                    Err(e)
                }
            }
    }

    async fn select_message(
        &self,
        pool: &PgPool,
        id: i64
    ) -> Result<Option<MessageWithFollowingAndBroadcastQueryResult>, sqlx::Error> {
        let message_result = query_as::<_, MessageWithProfileQueryResult>(
            r"
                select m.id, m.updated_at, m.body, m.likes, m.image, m.user_id, p.user_name, p.full_name, p.avatar, mb.id as message_broadcast_id                    
                    from message m 
                        join profile p on m.user_id = p.id
                        left join message_broadcast mb on m.id = mb.main_msg_id
                    where
                        m.id = $1
            "
            )
            .bind(id)
            .fetch_optional(pool).await;

        match message_result {
            Ok(message) => {
                if let Some(msg) = message {
                    let optional_matching_broadcast_message = get_broadcasting_message_of_message(
                        pool,
                        &msg
                    ).await;
                    let final_message = append_broadcast_msg_to_msg(
                        optional_matching_broadcast_message.as_ref(),
                        &msg
                    );
                    Ok(Some(final_message))
                } else {
                    Ok(None)
                }
            }
            Err(e) => Err(e),
        }
    }

    async fn select_messages(
        conn: &PgPool,
        user_id: i64,
        last_updated_at: DateTime<Utc>,
        page_size: i16
    ) -> Result<Vec<MessageWithFollowingAndBroadcastQueryResult>, sqlx::Error> {
        match query_as::<_, MessageWithProfileQueryResult>(
                r"
                select m.id, m.updated_at, m.body, m.likes, m.image, m.msg_group_type, m.user_id, p.user_name, p.full_name, p.avatar, mb.id as broadcast_msg_id                    
                    from message m 
                        join follow f on m.user_id = f.following_id
                        join profile p on p.id = f.following_id
                        left join message_broadcast mb on m.id = mb.main_msg_id
                        where
                            f.follower_id = $1 
                            and m.updated_at < $2
                        order by m.updated_at desc 
                        limit $3
            "
            )
            .bind(user_id)
            .bind(last_updated_at)
            .bind(page_size)
            .fetch_all(conn)
            .await {                
                Ok(following_messages) => {
                    let following_messages_with_broadcasts = following_messages                        
                        .iter()
                        .filter(|msg| {
                            msg.message_broadcast_id.is_some() && msg.message_broadcast_id.unwrap() > 0
                        })
                        .collect::<Vec<&MessageWithProfileQueryResult>>();

                    let optional_matching_broadcast_messages = get_broadcasting_messages_of_messages(
                        conn,
                        following_messages_with_broadcasts
                    ).await;
                    let final_message_list = append_broadcast_msgs_to_msgs(
                        &optional_matching_broadcast_messages,
                        &following_messages
                    );
                    Ok(final_message_list)
                }
                Err(e) => Err(e),
            }
    }
}

async fn get_broadcasting_messages_of_messages(
    conn: &PgPool,
    following_messages_with_broadcasts: Vec<&MessageWithProfileQueryResult>
) -> Option<Vec<MessageWithProfileQueryResult>> {
    let following_broadcast_message_ids = following_messages_with_broadcasts
        .iter()
        .map(|msg| { msg.message_broadcast_id.unwrap() })
        .collect::<Vec<i64>>();

    match query_as::<_, MessageWithProfileQueryResult>(
            r"
            select m.id, m.updated_at, m.body, m.likes, m.image, m.msg_group_type, m.user_id, p.user_name, p.full_name, p.avatar, mb.id as broadcast_msg_id
                from message m 
                    join profile p on m.user_id = p.id
                    left join message_broadcast mb on m.id = mb.main_msg_id
                where m.id = ANY($1)
        "
        )
        .bind(following_broadcast_message_ids)
        .fetch_all(conn)
        .await {            
            Ok(broadcast_messages) => { Some(broadcast_messages) }
            Err(e) => {
                error!("get_broadcasting_messages_of_messages: {}", e);
                None
            }
        }
}

async fn get_broadcasting_message_of_message(
    pool: &PgPool,
    message: &MessageWithProfileQueryResult
) -> Option<MessageWithProfileQueryResult> {
    match query_as::<_, MessageWithProfileQueryResult>(
            r"
            select m.id, m.updated_at, m.body, m.likes, m.image, m.user_id, p.user_name, p.full_name, p.avatar, mb.id as message_broadcast_id
                from message m 
                    join profile p on m.user_id = p.id
                    left join message_broadcast mb on m.id = mb.broadcasting_msg_id
                where mb.id = $1
        "
        )
        .bind(message.message_broadcast_id)
        .fetch_optional(pool)
        .await {
            Ok(broadcast_message) => broadcast_message,
            Err(e) => {
                error!("Error get_broadcasting_messages_of_messages: {}", e);
                None
            }
        }
}

fn append_broadcast_msgs_to_msgs(
    optional_broadcast_messages: &Option<Vec<MessageWithProfileQueryResult>>,
    following_messages_with_broadcasts: &Vec<MessageWithProfileQueryResult>
) -> Vec<MessageWithFollowingAndBroadcastQueryResult> {
    let mut final_list_of_messages: Vec<MessageWithFollowingAndBroadcastQueryResult> = vec![];

    following_messages_with_broadcasts.iter().for_each(|following_message_with_broadcast| {
        let matching_broadcast_msg = if
            let Some(broadcast_messages) = optional_broadcast_messages
        {
            broadcast_messages
                .iter()
                .find(|bm| { Some(bm.id) == following_message_with_broadcast.message_broadcast_id })
        } else {
            None
        };

        final_list_of_messages.push(
            append_broadcast_msg_to_msg(
                matching_broadcast_msg,
                following_message_with_broadcast
            )
        );
    });

    final_list_of_messages
}

fn append_broadcast_msg_to_msg(
    broadcast_message: Option<&MessageWithProfileQueryResult>,
    message_with_broadcast: &MessageWithProfileQueryResult
) -> MessageWithFollowingAndBroadcastQueryResult {
    let mut final_message = MessageWithFollowingAndBroadcastQueryResult {
        id: message_with_broadcast.id,
        updated_at: message_with_broadcast.updated_at,
        body: message_with_broadcast.body.clone(),
        likes: message_with_broadcast.likes,
        image: message_with_broadcast.image.clone(),
        user_id: message_with_broadcast.user_id,
        user_name: message_with_broadcast.user_name.clone(),
        full_name: message_with_broadcast.full_name.clone(),
        avatar: message_with_broadcast.avatar.clone(),
        message_broadcast_id: None,
        message_broadcast_updated_at: None,
        message_broadcast_user_id: None,
        message_broadcast_body: None,
        message_broadcast_likes: None,
        message_broadcast_image: None,
        message_broadcast_user_name: None,
        message_broadcast_full_name: None,
        message_broadcast_avatar: None,
    };

    if let Some(matching_broadcast) = broadcast_message {
        final_message.message_broadcast_id = Some(matching_broadcast.id);
        final_message.message_broadcast_updated_at = Some(matching_broadcast.updated_at);
        final_message.message_broadcast_body = matching_broadcast.body.to_owned();
        final_message.message_broadcast_likes = Some(matching_broadcast.likes);
        final_message.message_broadcast_image = matching_broadcast.image.to_owned();
        final_message.message_broadcast_user_id = Some(matching_broadcast.user_id);
        final_message.message_broadcast_user_name = Some(matching_broadcast.user_name.to_string());
        final_message.message_broadcast_full_name = Some(matching_broadcast.full_name.to_string());
        final_message.message_broadcast_avatar = matching_broadcast.avatar.to_owned();
    }

    final_message
}