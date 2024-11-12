use axum::{extract::Path, Json};
use crate::{controllers::lib::{errors::AppErrors, response::AppResponse}, repository::profiles::profile_model::Profile};
use super::profile_parameters::NewProfileRouteParam;
use fake::faker::lorem::en::Sentence;
use fake::{faker::internet::en::Username, Fake};
use fake::faker::name::en::{FirstName, LastName};

pub async fn create_profile(Json(_new_profile): Json<NewProfileRouteParam>) -> Result<i64, AppErrors> {
    Ok(1)
}

pub async fn get_profile(Path(_id): Path<i64>) -> Result<AppResponse<Profile>, AppErrors> {
    Ok(AppResponse::JsonData(Profile {
        id: 1,
        user_name: Username().fake(),
        full_name: format!("{} {}", FirstName().fake::<String>(), LastName().fake::<String>()),
        description: Sentence(2..2).fake(),
        region: "usa".to_string(),
        main_url: "http://test.com".to_string(),
        avatar: vec![]
    }))
}