use axum::{http::StatusCode, response::IntoResponse};

pub enum AppErrors {
    InternalServerError
}

impl IntoResponse for AppErrors {
    fn into_response(self) -> axum::response::Response {
        match self {
            AppErrors::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}