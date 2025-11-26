use axum::Extension;
use axum::extract::Path;
use axum::response::IntoResponse;
use axum::{Json, http::StatusCode};
use log::error;
use serde::Serialize;

use crate::model::{User, UserInfo};
use crate::user_service::UserService;

pub async fn list_users(service: Extension<UserService>) -> Result<Json<Vec<User>>, StatusCode> {
    match service.list_users().await {
        Ok(users) => Ok(Json(users)),
        Err(e) => {
            error!("Unexpected error: {:?}", e);
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
            error!("Unexpected error: {:?}", e);
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
            error!("Unexpected error: {:?}", e);
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
            error!("Unexpected error: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// pub async fn delete_user(
//     service: Extension<UserService>,
//     Path(id): Path<i32>,
// ) -> Result<Json<bool>, StatusCode> {
//     match service.delete_user(id).await {
//         Ok(_) => Ok(Json(true)),
//         Err(sqlx::Error::RowNotFound) => Err(StatusCode::NOT_FOUND), // Handle user not found case
//         Err(e) => {
//             error!("Unexpected error: {:?}", e);
//             Err(StatusCode::INTERNAL_SERVER_ERROR)
//         }
//     }
// }

#[derive(Serialize)]
pub struct ApiError {
    pub message: String,
}

pub async fn delete_user(
    service: Extension<UserService>,
    Path(id): Path<i32>,
) -> Result<Json<bool>, impl IntoResponse> {
    // Changed the error type
    match service.delete_user(id).await {
        Ok(_) => Ok(Json(true)),

        // **This is the main change:**
        Err(sqlx::Error::RowNotFound) => {
            let error_body = ApiError {
                message: format!("User with ID {} does not exist.", id),
            };

            // Return a tuple that implements IntoResponse: (StatusCode, Json Body)
            Err((StatusCode::NOT_FOUND, Json(error_body)))
        }

        Err(e) => {
            error!("Unexpected error: {:?}", e);

            // Return an Internal Server Error with a generic JSON message
            let error_body = ApiError {
                message: "Internal server error occurred.".to_string(),
            };
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_body)))
        }
    }
}
