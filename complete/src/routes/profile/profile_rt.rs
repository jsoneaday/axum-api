use std::sync::Arc;
use axum::{extract::State, routing::post, Router};
use crate::{controllers::profile::profile_ctrl::create_profile, lib::app_state::AppState};

pub fn get_profile_route(State(state): State<Arc<AppState>>) -> Router {
    Router::new()
        .route("/profile", post(create_profile))
        .with_state(state)
}