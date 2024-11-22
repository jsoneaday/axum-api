use std::sync::Arc;
use std::usize;
use axum::body::Body;
use axum::extract::{Request, State};
use complete::lib::app_state::AppState;
use complete::repository::profile::profile_models::ProfileQueryResult;
use complete::repository::repo::{DbRepo, EntityId};
use complete::routes::profile::profile_rt::get_profile_router;
use complete::test_utils::fixtures::init_test_logging;
use serde_json::json;
use fake::faker::internet::en::Username;
use fake::faker::name::en::{FirstName, LastName};
use fake::faker::lorem::en::Sentence;
use fake::Fake;
use tower::ServiceExt;

#[tokio::test]
async fn test_create_profile() {
    init_test_logging();
    let state = State(Arc::new(AppState {
        repo: DbRepo::init().await
    }));

    let user_name = Username().fake::<String>();
    let full_name = format!("{} {}", FirstName().fake::<String>(), LastName().fake::<String>());
    let description = Sentence(1..2).fake::<String>();
    let req_create_profile = Request::builder()
        .uri("/profile")
        .method("POST")
        .header("Content-Type", "application/json")
        .body(Body::from(
            json!({
                "user_name": user_name,
                "full_name": full_name,
                "description": description
            }).to_string()
        ))
        .unwrap();
    let profile_router = get_profile_router(state.clone());
    let res_create_profile = profile_router.clone().oneshot(req_create_profile).await.unwrap();    
    let profile_entity: EntityId = serde_json::from_slice(
        &axum::body::to_bytes(res_create_profile.into_body(), usize::MAX).await.unwrap()
    ).unwrap();
    assert!(profile_entity.id > 0);

    let req_profile = Request::builder()
        .uri(format!("/profile/{}", profile_entity.id))
        .method("GET")
        .body(Body::empty())
        .unwrap();
    let res_profile = profile_router.oneshot(req_profile).await.unwrap();
    let profile: ProfileQueryResult = serde_json::from_slice(
        &axum::body::to_bytes(res_profile.into_body(), usize::MAX).await.unwrap()
    ).unwrap();
    assert_eq!(profile.id, profile_entity.id);
    assert_eq!(profile.user_name, user_name);
    assert_eq!(profile.full_name, full_name);
    assert_eq!(profile.description, description);
}