use std::sync::Arc;
use axum::{extract::State, routing::{get, post}, Router};
use crate::{controllers::message::message_ctrl::{create_message, get_message}, lib::app_state::AppState};

pub fn get_message_routes(State(state): State<Arc<AppState>>) -> Router {
    Router::new()
        .route("/message", post(create_message))
        .route("/message/:id", get(get_message))
        .with_state(state)
}