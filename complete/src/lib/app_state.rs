use crate::repository::repo::DbRepo;

pub struct CurrentUser {
    pub user_name: String,
    pub full_name: String
}

#[derive(Clone)]
pub struct AppState {
    pub repo: DbRepo
}