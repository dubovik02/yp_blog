use sqlx::{PgPool};

use crate::domain::{error::ServerError, user::{User}};


pub struct UserRepository {
    pub pool: PgPool,
}

impl UserRepository {

    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn insert_user(&self, username: String, email: String, password: String) -> Result<User, ServerError> {

        match sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (username, email, password_hash, created_at)
            VALUES ($1, $2, $3, NOW())
            RETURNING id, username, email, password_hash, created_at
            "#,
            username,
            email,
            password
        )
        .fetch_optional(&self.pool)
        .await {
            Ok(new_user_opt) => {
                match new_user_opt {
                    Some(user) => Ok(user),
                    None => Err(ServerError::UserAlreadyExistsError),
                }
            },
            Err(_) => {
                Err(ServerError::UserAlreadyExistsError)
            },
        }
    }

    pub async fn get_user_by_email(&self, email: String) -> Result<User, ServerError> {

        match sqlx::query_as!(
            User,
            r#"
            SELECT * FROM users WHERE email = $1
            "#,
            email
        )
        .fetch_optional(&self.pool)
        .await {
            Ok(user) => {
                match user {
                    Some(user) => Ok(user),
                    None => Err(ServerError::UserNotFoundError)
                }
            },
            Err(e) => {
                Err(ServerError::InternalServerError(Box::new(e)))
            },
        }
    }
}

