use std::sync::Arc;
use tracing::error;
use axum::response::{IntoResponse, Response};
use axum::extract::{Path, State};
use axum::Json;
use crate::lib::app_state::AppState;
use crate::repository::profile::profile_repo::{InsertProfileFn, SelectProfileFn};
use crate::repository::repo::Repository;
use crate::routes::lib::app_response::AppResponse;
use crate::routes::lib::error::AppErrors;
use super::profile_models::CreateProfile;


pub async fn create_profile(State(state): State<Arc<AppState>>, Json(create_profile): Json<CreateProfile>) -> Response {
    // todo: add auth middleware

    let app_state = Arc::clone(&state);
    match app_state.repo.insert_profile(
        app_state.repo.get_pool(), 
        create_profile.user_name,
        create_profile.full_name,
        create_profile.description,
        create_profile.region,
        create_profile.main_url,
        create_profile.avatar
    ).await {
        Ok(entity) => AppResponse::Create(entity).into_response(),
        Err(e) => {
            error!("Error failed insert_profile {:?}", e);
            AppErrors::InternalServerError.into_response()
        }
    }    
}

pub async fn get_profile(State(state): State<Arc<AppState>>, Path(id): Path<i64>) -> Response {
    let app_state = Arc::clone(&state);
    match app_state.repo.select_profile(app_state.repo.get_pool(), id).await {
        Ok(profile) => AppResponse::JsonData(profile).into_response(),
        Err(e) => {
            error!("Error failed get_profile {:?}", e);   
            AppErrors::InternalServerError.into_response()
        }
    }
}