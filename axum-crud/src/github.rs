use axum::{extract::Path, routing::get, Json, Router};

use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;

const BASE_GITHUB_API_URL: &str = "https://api.github.com";

pub fn get_router() -> Router {
    let github_api_client = Client::new();

    Router::new()
        .with_state(github_api_client)
        .route("/gh/user/:username", get(get_user_info))
}

#[derive(Serialize, Deserialize)]
struct GithubUserInfo {
    avatar_url: String,
    bio: Option<String>,
    blog: Option<String>,
    company: Option<String>,
    created_at: String,
    email: String,
    followers: isize,
    following: isize,
    id: isize,
    location: String,
    // #[serde(rename(serialize = "username", deserialize = "login"))]
    login: String,
    name: String,
    public_gists: isize,
    public_repos: isize,
}

async fn get_user_info(Path(username): Path<String>) -> Result<Json<Value>, String> {
    let response = reqwest::get(format!("{BASE_GITHUB_API_URL}/users/{username}"))
        .await
        .map_err(|e| e.to_string())?
        .json::<Value>()
        .await
        .map_err(|e| e.to_string())?;

    println!("{}", response);

    Ok(Json(response))
}
