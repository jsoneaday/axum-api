use axum::{http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, prelude::FromRow, PgPool};
use std::env;
use dotenv::dotenv;

#[derive(Serialize, Deserialize, FromRow)]
pub struct EntityId {
    pub id: i64
}

impl IntoResponse for EntityId {
    fn into_response(self) -> axum::response::Response {
        match serde_json::to_string(&self) {
            Ok(self_str) => (StatusCode::CREATED, self_str).into_response(),
            Err(_) => (StatusCode::INTERNAL_SERVER_ERROR).into_response()
        }        
    }
}

#[derive(Clone)]
pub struct DbRepo {
    pool: PgPool
}

impl DbRepo {
    pub async fn init() -> Self {
        Self {
            pool: get_coon().await
        }
    }
}

pub trait Repository {
    fn get_pool(&self) -> &PgPool;
}

impl Repository for DbRepo {
    fn get_pool(&self) -> &PgPool {
        &self.pool
    }
}

async fn get_coon() -> PgPool {
    dotenv().ok();

    let host = env::var("POSTGRES_HOST").unwrap();
    let port = env::var("POSTGRES_PORT").unwrap();
    let user_name = env::var("POSTGRES_USER").unwrap();
    let password = env::var("POSTGRES_PASSWORD").unwrap();
    let db = env::var("POSTGRES_DB").unwrap();

    let conn_str = format!("postgres://{}:{}@{}:{}/{}", user_name, password, host, port, db);

    PgPoolOptions::new().max_connections(5).connect(&conn_str).await.unwrap()
}