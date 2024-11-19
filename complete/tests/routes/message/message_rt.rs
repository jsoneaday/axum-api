use std::sync::Arc;
use std::usize;
use axum::body::Body;
use axum::extract::State;
use axum::http::{Request, StatusCode};
use complete::lib::app_state::AppState;
use complete::repository::message::message_models::{MessageQueryResult, MessageWithFollowingAndBroadcastQueryResult};
use complete::repository::profile::profile_models::ProfileQueryResult;
use complete::repository::repo::{DbRepo, EntityId};
use complete::routes::message::message_rt::get_message_routes;
use complete::routes::profile::profile_rt::get_profile_routes;
use complete::testing::fixtures::init_test_logging;
use tower::ServiceExt;
use serde_json::json;
use fake::faker::internet::en::{FreeEmail, Username};
use fake::faker::name::en::{FirstName, LastName};
use fake::faker::lorem::en::Sentence;
use fake::faker::address::en::CountryName;
use fake::Fake;
use tracing::info;

#[tokio::test]
async fn test_insert_message() {
    init_test_logging();

    let state = State(Arc::new(AppState {
        repo: DbRepo::init().await
    }));    

    let req_insert_profile = Request::builder()
        .uri("/profile")
        .method("POST")
        .header("Content-Type", "application/json")
        .body(Body::from(json!({
            "user_name": Username().fake::<String>(),
            "full_name": format!("{} {}", FirstName().fake::<String>(), LastName().fake::<String>()),
            "description": Sentence(1..2).fake::<String>()
        }).to_string()))
        .unwrap();   
    let res_profile = get_profile_routes(state.clone()).oneshot(req_insert_profile).await.unwrap();
    let profile: EntityId = serde_json::from_slice(
        &axum::body::to_bytes(res_profile.into_body(), usize::MAX).await.unwrap()
    ).unwrap();

    let new_message = Sentence(1..2).fake::<String>();
    let message_router = get_message_routes(state.clone());
    let req_insert_message = Request::builder()
        .uri("/message")
        .method("POST")
        .header("Content-Type", "application/json")
        .body(Body::from(json!({
            "user_id": profile.id,
            "body": new_message.clone()
        }).to_string()))
        .unwrap();
    let res_insert_message = message_router.clone().oneshot(req_insert_message).await.unwrap();
    assert_eq!(res_insert_message.status(), StatusCode::CREATED);    
    let message_entity: EntityId = serde_json::from_slice(
        &axum::body::to_bytes(res_insert_message.into_body(), usize::MAX).await.unwrap()
    ).unwrap();
    assert!(message_entity.id > 0);

    let req_message = Request::builder()
        .uri(format!("/message/{}", message_entity.id))
        .method("GET")
        .body(Body::empty())
        .unwrap();
    let res_message = message_router.oneshot(req_message).await.unwrap();
    let message: MessageWithFollowingAndBroadcastQueryResult = serde_json::from_slice(
        &axum::body::to_bytes(res_message.into_body(), usize::MAX).await.unwrap()
    ).unwrap();
    assert!(message.body.unwrap() == new_message);
}