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

use axum::Router;
use dotenv::dotenv;

pub async fn run() {
    dotenv().ok();

    _ = axum::serve(
        tokio::net::TcpListener::bind("0.0.0.0:4000").await.unwrap(),
        Router::new()
    ).await;
}