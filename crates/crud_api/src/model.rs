use serde::{Deserialize, Serialize};

/// CREATE TABLE users (id INTEGER PRIMARY KEY, name VARCHAR(255), age INTERGER, email VARCHAR(255));
#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub age: u8,
    pub email: String,
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct UserInfo {
    pub name: String,
    pub age: u8,
    pub email: String,
}
