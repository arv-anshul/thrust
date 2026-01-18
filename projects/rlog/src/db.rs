use diesel::{connection::SimpleConnection, prelude::*};
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut conn = SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

    // Force foreign keys on
    conn.batch_execute("PRAGMA foreign_keys = ON;")
        .expect("Failed to enable foreign keys");

    conn
}
