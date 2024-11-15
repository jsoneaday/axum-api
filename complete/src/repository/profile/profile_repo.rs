use crate::repository::repo::{DbRepo, EntityId};
use sqlx::error::Error;
use sqlx::query_as;
use sqlx::PgPool;
use super::profile_models::ProfileQueryResult;
use async_trait::async_trait;

#[async_trait]
pub trait InsertProfileFn {
    async fn insert_profile(
        &self, 
        pool: &PgPool, 
        user_name: String,
        full_name: String,
        description: String,
        region: Option<String>,
        main_url: Option<String>,
        avatar: Option<Vec<u8>>
    ) -> Result<EntityId, Error>;
}

#[async_trait]
impl InsertProfileFn for DbRepo {
    async fn insert_profile(
        &self, 
        pool: &PgPool, 
        user_name: String,
        full_name: String,
        description: String,
        region: Option<String>,
        main_url: Option<String>,
        avatar: Option<Vec<u8>>
    ) -> Result<EntityId, Error> {
        query_as::<_, EntityId>(r"
            insert into Profile
            (user_name, full_name, description, region, main_url, avatar)
            values
            ($1, $2, $3, $4, $5, $6)
            returning id
        ")
        .bind(user_name)
        .bind(full_name)
        .bind(description)
        .bind(region)
        .bind(main_url)
        .bind(avatar)
        .fetch_one(pool)
        .await
    }
}

#[async_trait]
pub trait SelectProfileFn {
    async fn select_profile(&self, pool: &PgPool, id: i64) -> Result<Option<ProfileQueryResult>, Error>;
}

#[async_trait]
impl SelectProfileFn for DbRepo {
    async fn select_profile(&self, conn: &PgPool, id: i64) -> Result<Option<ProfileQueryResult>, Error> {
        query_as::<_, ProfileQueryResult>("select * from profile where id = $1")
            .bind(id)
            .fetch_optional(conn)
            .await
    }
}
