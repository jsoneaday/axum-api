pub mod controllers {
    pub mod message {
        pub mod message_models;
        pub mod message_ctrl;
    }
    pub mod profile {        
        pub mod profile_models;
        pub mod profile_ctrl;
    }
}
pub mod routes {
    pub mod lib {
        pub mod error;
        pub mod app_response;
    }
    pub mod message {
        pub mod message_rt;
    }
    pub mod profile {
        pub mod profile_rt;
    }
}
pub mod lib {
    pub mod app_state;
}
pub mod repository {
    pub mod repo;
    pub mod message {
        pub mod message_models;
        pub mod message_repo;
    }
    pub mod profile {
        pub mod profile_models;
        pub mod profile_repo;
    }
}

use std::sync::Arc;
use std::env;
use axum::{extract::State, Router};
use dotenv::dotenv;
use lib::app_state::AppState;
use repository::repo::DbRepo;
use routes::{message::message_rt::get_message_routes, profile::profile_rt::get_profile_routes};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

pub async fn run() {
    dotenv().ok();

    let host = env::var("HOST").unwrap();
    let port = env::var("PORT").unwrap().parse::<u16>().unwrap();

    let tracing_sub = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(tracing_sub)
        .expect("Setting default subscriber failed");

    let state = State(Arc::new(AppState {
        repo: DbRepo::init().await
    }));

    info!("Server starting at {}:{}", host, port);
    _ = axum::serve(
        tokio::net::TcpListener::bind(format!("{}:{}", host, port)).await.unwrap(),
        Router::new()
            .merge(get_profile_routes(state.clone()))
            .merge(get_message_routes(state))
    ).await;
}