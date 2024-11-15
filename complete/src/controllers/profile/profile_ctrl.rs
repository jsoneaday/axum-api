use axum::response::{IntoResponse, Response};
use axum::extract::State;
use axum::Json;
use crate::lib::app_state::AppState;
use crate::repository::profile::profile_repo::InsertProfileFn;
use crate::repository::repo::Repository;
use crate::routes::lib::app_response::AppResponse;
use crate::routes::lib::error::AppErrors;
use std::sync::Arc;
use super::profile_models::{convert_new_profile, CreateProfile};
// use fake::faker::internet::en::Username;
// use fake::faker::name::en::{FirstName, LastName};
// use fake::faker::lorem::en::Sentence;
// use fake::faker::address::en::CountryName;
// use fake::Fake;


pub async fn create_profile(State(state): State<Arc<AppState>>, Json(create_profile): Json<CreateProfile>) -> Response {
    // todo: add auth middleware

    let app_state = Arc::clone(&state);
    match app_state.repo.insert_profile(app_state.repo.get_pool(), convert_new_profile(create_profile)).await {
        Ok(entity) => AppResponse::Create(entity.id).into_response(),
        Err(_) => AppErrors::InternalServerError.into_response()
    }    
}