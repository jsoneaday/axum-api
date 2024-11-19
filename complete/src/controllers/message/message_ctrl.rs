use std::sync::Arc;
use axum::response::{IntoResponse, Response};
use axum::extract::{Json, Path, State};
use tracing::error;
use crate::lib::app_state::AppState;
use crate::repository::message::message_repo::MessageRepo;
use crate::repository::repo::Repository;
use crate::routes::lib::app_response::AppResponse;
use crate::routes::lib::error::AppErrors;
use super::message_models::CreateMessage;

pub async fn create_message(State(state): State<Arc<AppState>>, Json(create_message): Json<CreateMessage>) -> Response {
    let app_state = Arc::clone(&state);
    match app_state.repo.insert_message(app_state.repo.get_pool(), create_message.user_id, &create_message.body, create_message.broadcasting_msg_id).await {
        Ok(entity) => AppResponse::Create(entity).into_response(),
        Err(e) => {
            error!("Error failed create_message {:?}", e);
            AppErrors::InternalServerError.into_response()
        }
    }
}

pub async fn get_message(State(state): State<Arc<AppState>>, Path(id): Path<i64>) -> Response {
    let app_state = Arc::clone(&state);
    match app_state.repo.select_message(app_state.repo.get_pool(), id).await {
        Ok(msg) => AppResponse::JsonData(msg).into_response(),
        Err(e) => {
            error!("Error get_message {:?}", e);
            AppErrors::InternalServerError.into_response()
        }
    }
}