use axum::Extension;
use axum::extract::Path;
use axum::{Json, http::StatusCode};

use crate::model::{User, UserInfo};
use crate::user_service::UserService;

pub async fn list_users(service: Extension<UserService>) -> Result<Json<Vec<User>>, StatusCode> {
    match service.list_users().await {
        Ok(users) => Ok(Json(users)),
        Err(e) => {
            eprintln!("Unexpected error: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn create_user(
    service: Extension<UserService>,
    Json(user): Json<UserInfo>,
) -> Result<Json<bool>, StatusCode> {
    match service.create_user(user).await {
        Ok(_) => Ok(Json(true)),
        Err(e) => {
            eprintln!("Unexpected error: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_user_by_id(
    service: Extension<UserService>,
    Path(id): Path<i32>,
) -> Result<Json<UserInfo>, StatusCode> {
    match service.get_user_by_id(id).await {
        Ok(user) => Ok(Json(user)),
        Err(sqlx::Error::RowNotFound) => Err(StatusCode::NOT_FOUND), // Handle user not found case
        Err(e) => {
            eprintln!("Unexpected error: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn update_user(
    service: Extension<UserService>,
    Path(id): Path<i32>,
    Json(user): Json<UserInfo>,
) -> Result<Json<UserInfo>, StatusCode> {
    match service.update_user(id, user).await {
        Ok(user) => Ok(Json(user)),
        Err(e) => {
            eprintln!("Unexpected error: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn delete_user(
    service: Extension<UserService>,
    Path(id): Path<i32>,
) -> Result<Json<bool>, StatusCode> {
    match service.delete_user(id).await {
        Ok(_) => Ok(Json(true)),
        Err(sqlx::Error::RowNotFound) => Err(StatusCode::NOT_FOUND), // Handle user not found case
        Err(e) => {
            eprintln!("Unexpected error: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
