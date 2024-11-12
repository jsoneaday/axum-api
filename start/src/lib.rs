pub mod controllers {
    pub mod profiles {
        pub mod profile_controller;
        pub mod profile_parameters;
    }
    pub mod lib {
        pub mod errors;
        pub mod response;
    }
}
pub mod repository {
    pub mod profiles {
        pub mod profile_repo;
        pub mod profile_model;
    }
    pub mod repo;
}
pub mod routes {
    pub mod profiles_router;
    pub mod messages_router;
}
pub mod lib {
    pub mod app_state;
}

use axum::Router;
use lib::app_state::AppState;
use repository::repo::DbRepo;
use routes::profiles_router::get_profile_router;

pub async fn run() {
    let state = AppState { repo: DbRepo::init().await };

    let router = Router::new().merge(get_profile_router(state));
    
    axum::serve(
        tokio::net::TcpListener::bind("0.0.0.0:4000").await.unwrap(),
        router
    )
    .await
    .unwrap();        
}
