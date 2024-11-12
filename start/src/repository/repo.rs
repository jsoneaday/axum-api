#![allow(unused)]
use sqlx::{postgres::PgPoolOptions, prelude::FromRow, PgPool};
use dotenv::dotenv;
use std::env;

#[derive(FromRow)]
pub struct EntityId {
    pub id: i64
}

#[derive(Clone)]
pub struct DbRepo {
    pub pool: PgPool
}

impl DbRepo {
    pub async fn init() -> Self {
        Self {
            pool: get_conn_pool().await
        }
    }
}

pub trait Repository {
    type Output;

    fn get_conn(&self) -> &Self::Output;
}

impl Repository for DbRepo {
    type Output = PgPool;

    fn get_conn(&self) -> &Self::Output {
        &self.pool    
    }
}

async fn get_conn_pool() -> PgPool {
    dotenv().ok();
    let postgres_host = env::var("POSTGRES_HOST").unwrap();
    let postgres_port = env::var("POSTGRES_PORT").unwrap().parse::<u16>().unwrap();
    let postgres_password = env::var("POSTGRES_PASSWORD").unwrap();
    let postgres_user = env::var("POSTGRES_USER").unwrap();
    let postgres_db = env::var("POSTGRES_DB").unwrap();
    let postgres_url = format!(
        "postgres://{postgres_user}:{postgres_password}@{postgres_host}:{postgres_port}/{postgres_db}"
    );

    PgPoolOptions::new().max_connections(5).connect(&postgres_url).await.unwrap()
}