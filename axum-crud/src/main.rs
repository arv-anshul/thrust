pub mod github;

use axum::{extract::Query, http::StatusCode, response::Html, routing::get, Json, Router};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[tokio::main]
async fn main() {
    let github_routes = github::get_router();
    let app = Router::new()
        .route(
            "/",
            get(|| async { json!({"message": "Hello, World!"}).to_string() }),
        )
        .route("/author/get", get(get_author))
        .route("/author/greet", get(greet_author))
        .merge(github_routes);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();
    eprintln!("listening at http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

#[derive(Serialize, Deserialize)]
struct AuthorInfo {
    name: Option<String>,
    github: Option<String>,
}

async fn get_author() -> Json<AuthorInfo> {
    let author_info = AuthorInfo {
        name: Some("Anshul Raj Verma".to_string()),
        github: Some("arv-anshul".to_string()),
    };
    Json(author_info)
}

async fn greet_author(
    Query(author_info): Query<AuthorInfo>,
) -> Result<(StatusCode, Html<String>), (StatusCode, String)> {
    // just for sake handling the error manually
    if author_info.name.is_none() || author_info.github.is_none() {
        return Err((
            StatusCode::BAD_REQUEST,
            json!({"error": "Missing name or github in query params"}).to_string(),
        ));
    }

    let message = format!(
        "Hi! {}, I have seen your GitHub (https://github.com/{}) it's nice. Keep it up 👍",
        author_info.name.as_ref().unwrap(),
        author_info.github.as_ref().unwrap()
    );

    Ok((StatusCode::OK, Html(message)))
}
