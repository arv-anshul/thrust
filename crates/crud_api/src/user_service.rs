use crate::model::{User, UserInfo};
use log::{error, info, trace};
use sqlx::{Error, Executor, Sqlite, SqlitePool, migrate::MigrateDatabase};

#[derive(Clone)]
pub struct UserService {
    pool: SqlitePool,
}

impl UserService {
    pub async fn new(url: &str) -> Result<Self, Error> {
        if !Sqlite::database_exists(url).await.unwrap_or(false) {
            Sqlite::create_database(url).await?;
        }

        let pool = SqlitePool::connect(url).await?;
        trace!("Connected with database!");

        // Add dummy data into database
        UserService::add_dummy_data(&pool).await?;

        Ok(Self { pool })
    }

    async fn add_dummy_data(pool: &SqlitePool) -> Result<(), Error> {
        trace!("Creating dummy database.");

        // Create the table if it doesn't exist
        pool.execute(
            "CREATE TABLE IF NOT EXISTS users (
                id INTEGER PRIMARY KEY,
                name VARCHAR(255),
                age INTEGER,
                email VARCHAR(255)
            );",
        )
        .await?;

        // If table has already some data then don't append
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users")
            .fetch_one(pool)
            .await?;
        if count > 0 {
            info!("Skip adding dummy data.");
            return Ok(());
        }

        // Insert multiple users using execute()
        pool.execute(
            "INSERT INTO users (name, age, email) VALUES
            ('Alice Johnson', 28, 'alice@example.com'),
            ('Bob Smith', 34, 'bob@example.com'),
            ('Charlie Brown', 22, 'charlie@example.com'),
            ('David Wilson', 40, 'david@example.com'),
            ('Emma Davis', 29, 'emma@example.com'),
            ('Frank Miller', 37, 'frank@example.com'),
            ('Grace Lee', 25, 'grace@example.com'),
            ('Henry Adams', 45, 'henry@example.com'),
            ('Ivy Carter', 31, 'ivy@example.com'),
            ('Jack Turner', 27, 'jack@example.com');",
        )
        .await?;

        Ok(())
    }

    pub async fn list_users(&self) -> Result<Vec<User>, Error> {
        let users = sqlx::query_as::<_, User>("SELECT * FROM users;")
            .fetch_all(&self.pool)
            .await?;

        Ok(users)
    }

    pub async fn get_user_by_id(&self, id: i32) -> Result<UserInfo, Error> {
        let user = sqlx::query_as::<_, UserInfo>("SELECT * FROM users WHERE id=$1; DROP id;")
            .bind(id)
            .fetch_one(&self.pool)
            .await?;

        Ok(user)
    }

    pub async fn create_user(&self, user: UserInfo) -> Result<(), Error> {
        sqlx::query("INSERT INTO users (name, age, email) VALUES ($1, $2, $3)")
            .bind(user.name)
            .bind(user.age)
            .bind(user.email)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn update_user(&self, id: i32, user: UserInfo) -> Result<UserInfo, Error> {
        // Update user info
        sqlx::query("UPDATE users SET name = $2, age = $3, email = $4 WHERE id = $1")
            .bind(id)
            .bind(user.name)
            .bind(user.age)
            .bind(user.email)
            .execute(&self.pool)
            .await?;

        // Fetch the updated data to return
        let updated_user = sqlx::query_as::<_, UserInfo>("SELECT * FROM users WHERE id=$1")
            .bind(id)
            .fetch_one(&self.pool)
            .await?;

        Ok(updated_user)
    }

    pub async fn delete_user(&self, id: i32) -> Result<(), Error> {
        let exists: bool = sqlx::query_scalar("SELECT EXISTS(SELECT * FROM users WHERE id=$1);")
            .bind(id)
            .fetch_one(&self.pool)
            .await?;

        // Return an error if the ID doesn't exist in the database
        if !exists {
            error!("Row not found for user id={id}!");
            return Err(Error::RowNotFound);
        }

        sqlx::query("DELETE FROM users WHERE id=$1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
