mod controller;
mod model;
mod user_service;
use axum::Extension;
use axum::Router;
use axum::routing::*;
use user_service::UserService;

#[tokio::main]
async fn main() {
    println!("Starting server!");

    // Added `.arv.` in the db filename to ignore from git
    let user_service = match UserService::new("sqlite://data.arv.sqlite").await {
        Ok(service) => service,
        Err(e) => panic!("Database error: {:?}", e),
    };
    let app = Router::new()
        .route("/", get(async || "Hello, World!"))
        .route("/users", get(controller::list_users))
        .route("/user", post(controller::create_user))
        .route("/user/{id}", get(controller::get_user_by_id))
        .route("/user/{id}", put(controller::update_user))
        .route("/user/{id}", delete(controller::delete_user))
        .layer(Extension(user_service));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    println!(
        "Listening at port http://localhost:{}",
        listener.local_addr().unwrap().port()
    );

    // Serve the axum server at port 8000
    axum::serve(listener, app).await.unwrap();
}
