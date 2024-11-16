use async_trait::async_trait;
use sqlx::{Error, PgPool};
use sqlx::query_as;
use crate::repository::repo::{DbRepo, EntityId};
use tracing::error;
use super::message_models::{MessageWithFollowingAndBroadcastQueryResult, MessageWithProfileQueryResult};

#[async_trait]
pub trait InsertMessageFn {
    async fn insert_message(&self, pool: &PgPool, user_id: i64, body: &str, broadcasting_msg_id: Option<i64>) -> Result<EntityId, Error>;
}

#[async_trait]
impl InsertMessageFn for DbRepo {
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
}

#[async_trait]
pub trait SelectMessageFn {
    async fn select_message(&self, pool: &PgPool, id: i64) -> Result<Option<MessageWithFollowingAndBroadcastQueryResult>, Error>;
}

#[async_trait]
impl SelectMessageFn for DbRepo {
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
}

async fn get_broadcasting_message_of_message(
    pool: &PgPool,
    message: &MessageWithProfileQueryResult
) -> Option<MessageWithProfileQueryResult> {
    let broadcasting_msg_result = query_as::<_, MessageWithProfileQueryResult>(
            r"
            select m.id, m.updated_at, m.body, m.likes, m.image, m.user_id, p.user_name, p.full_name, p.avatar, mb.id as message_broadcast_id
                from message m 
                    join profile p on m.user_id = p.id
                    left join message_broadcast mb on m.id = mb.broadcasting_msg_id
                where mb.id = $1
        "
        )
        .bind(message.message_broadcast_id)
        .fetch_optional(pool).await;

    match broadcasting_msg_result {
        Ok(broadcast_message) => broadcast_message,
        Err(e) => {
            error!("Error get_broadcasting_messages_of_messages: {}", e);
            None
        }
    }
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