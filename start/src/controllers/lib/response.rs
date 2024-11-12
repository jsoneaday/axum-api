use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;

pub enum AppResponse<T: Serialize> {
    Ok,
    Created(i64),
    JsonData(T)
}

impl<T: Serialize> IntoResponse for AppResponse<T> {
    fn into_response(self) -> axum::response::Response {
        match self {
            AppResponse::Ok => (StatusCode::OK).into_response(),
            AppResponse::Created(id) => (StatusCode::CREATED, Json(id)).into_response(),
            AppResponse::JsonData(data) => (StatusCode::OK, Json(data)).into_response()
        }
    }
}