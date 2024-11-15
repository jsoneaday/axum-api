use crate::repository::repo::DbRepo;

#[derive(Clone)]
pub struct AppState {
    pub repo: DbRepo
}