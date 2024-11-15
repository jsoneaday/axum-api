use serde::Deserialize;
use crate::repository::profile::profile_models::NewProfile;

#[derive(Deserialize)]
pub struct CreateProfile {
    pub user_name: String,
    pub full_name: String,
    pub description: String,
    pub region: Option<String>,
    pub main_url: Option<String>,
    pub avatar: Option<Vec<u8>>,
}

pub fn convert_new_profile(create_profile: CreateProfile) -> NewProfile {
    NewProfile {
        user_name: create_profile.user_name,
        full_name: create_profile.full_name,
        description: create_profile.description,
        region: create_profile.region,
        main_url: create_profile.main_url,
        avatar: create_profile.avatar,
    }
}