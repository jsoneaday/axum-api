use axum::{routing::get, Router};
use crate::{controllers::profiles::profile_controller::get_profile, AppState};

pub fn get_profile_router(state: AppState) -> Router {
    Router::new()
        .route(
            "/profile",
            get(get_profile)
        )
        .with_state(state)
}