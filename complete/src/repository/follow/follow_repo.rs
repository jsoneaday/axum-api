use sqlx::PgPool;
use async_trait::async_trait;
use crate::repository::repo::{DbRepo, EntityId};

use super::follow_models::Follow;

#[async_trait]
pub trait FollowRepo {
    async fn insert_follow(conn: &PgPool, follower_id: i64, following_id: i64) -> Result<EntityId, sqlx::Error>;

    async fn select_follows_by_follower(pool: &PgPool, id: i64) -> Result<Vec<Follow>, sqlx::Error>;
}

#[async_trait]
impl FollowRepo for DbRepo {
    async fn insert_follow(conn: &PgPool, follower_id: i64, following_id: i64) -> Result<EntityId, sqlx::Error> {
        sqlx::query_as::<_, EntityId>(
                "insert into follow (follower_id, following_id) values ($1, $2) returning id"
            )
            .bind(follower_id)
            .bind(following_id)
            .fetch_one(conn)
            .await
    }

    async fn select_follows_by_follower(pool: &PgPool, id: i64) -> Result<Vec<Follow>, sqlx::Error> {
        sqlx::query_as::<_, Follow>(r"
            select * from follow
            where follower_id = $1
        ")
        .bind(id)
        .fetch_all(pool)
        .await
    }
}
