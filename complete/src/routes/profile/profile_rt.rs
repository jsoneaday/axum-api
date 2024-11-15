use std::sync::Arc;
use axum::{extract::State, routing::{get, post}, Router};
use crate::{controllers::profile::profile_ctrl::{create_profile, get_profile}, lib::app_state::AppState};

pub fn get_profile_routes(State(state): State<Arc<AppState>>) -> Router {
    Router::new()
        .route("/profile", post(create_profile))
        .route("/profile/:id", get(get_profile))
        .with_state(state)
}