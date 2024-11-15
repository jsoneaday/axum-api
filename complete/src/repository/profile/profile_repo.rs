use crate::repository::repo::{DbRepo, EntityId};
use sqlx::error::Error;
use sqlx::query_as;
use sqlx::PgPool;
use super::profile_models::NewProfile;
use super::profile_models::ProfileQueryResult;

pub trait InsertProfileFn {
    async fn insert_profile(&self, pool: &PgPool, new_profile: NewProfile) -> Result<EntityId, Error>;
}

impl InsertProfileFn for DbRepo {
    async fn insert_profile(&self, pool: &PgPool, new_profile: NewProfile) -> Result<EntityId, Error> {
        query_as::<_, EntityId>(r"
            insert into Profile
            (user_name, full_name, description, region, main_url, avatar)
            values
            ($1, $2, $3, $4, $5, $6)
            returning id
        ")
        .bind(&new_profile.user_name)
        .bind(&new_profile.full_name)
        .bind(&new_profile.description)
        .bind(&new_profile.region)
        .bind(&new_profile.main_url)
        .bind(&new_profile.avatar)
        .fetch_one(pool)
        .await
    }
}

pub trait SelectProfileFn {
    async fn select_profile(&self, pool: &PgPool, id: i64) -> Result<Option<ProfileQueryResult>, Error>;
}

impl SelectProfileFn for DbRepo {
    async fn select_profile(&self, conn: &PgPool, id: i64) -> Result<Option<ProfileQueryResult>, Error> {
        query_as::<_, ProfileQueryResult>("select * from profile where id = $1")
            .bind(id)
            .fetch_optional(conn)
            .await
    }
}
