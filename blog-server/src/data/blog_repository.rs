use sqlx::PgPool;

use crate::domain::{error::ServerError, 
    post::{Post, PostCreatedInfo, PostDeleteInfo, PostUpdateInfo}};

pub struct BlogRepository {
    pub pool: PgPool,
}

impl BlogRepository {

    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn insert_post(&self, post_data: PostCreatedInfo) -> Result<Post, ServerError> {

        match sqlx::query_as!(
            Post,
            r#"
            INSERT INTO posts (title, content, author_id, created_at, updated_at)
            VALUES ($1, $2, $3, NOW(), NOW())
            RETURNING id, title, content, author_id, created_at, updated_at
            "#,
            post_data.title,
            post_data.content,
            post_data.author_id,
        )
        .fetch_optional(&self.pool)
        .await {
            Ok(new_post_opt) => {
                match new_post_opt {
                    Some(new_post) => Ok(new_post),
                    None => Err(ServerError::PostNotFoundError),
                }
            },
            Err(e) => {
                Err(ServerError::InternalServerError(Box::new(e)))
            },
        }
    }

    pub async fn get_post_by_id(&self, post_id: i64) -> Result<Post, ServerError> {

        match sqlx::query_as!(
            Post,
            r#"
            SELECT * FROM posts WHERE id = $1
            "#,
            post_id,
        )
        .fetch_optional(&self.pool)
        .await {
            Ok(post_opt) => {
                match post_opt {
                    Some(post) => Ok(post),
                    None => Err(ServerError::PostNotFoundError),
                }
            },
            Err(e) => {
                Err(ServerError::InternalServerError(Box::new(e)))
            },
        }
    }

    pub async fn update_post_by_id(&self, post_id: i64,  post_data: PostUpdateInfo) -> Result<Post, ServerError> {

        match sqlx::query_as!(
            Post,
            r#"
            UPDATE posts SET title=$1, content=$2, updated_at=NOW() WHERE id=$3
            RETURNING id, title, content, author_id, created_at, updated_at
            "#,
            post_data.title,
            post_data.content,
            post_id,
        )
        .fetch_optional(&self.pool)
        .await {
            Ok(post_opt) => {
                match post_opt {
                    Some(post) => Ok(post),
                    None => Err(ServerError::PostNotFoundError),
                }
            },
            Err(e) => {
                Err(ServerError::InternalServerError(Box::new(e)))
            },
        }
    }

    pub async fn delete_post_by_id(&self, post_data: PostDeleteInfo) -> Result<bool, ServerError> {

        let result = sqlx::query!(
            "DELETE FROM posts WHERE id=$1 and author_id=$2",
            post_data.id,
            post_data.author_id
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn get_posts_list(&self, offset: i64, limit: i64) -> Result<Vec<Post>, ServerError> {

        match sqlx::query_as!(
            Post,
            r#"
            SELECT * FROM posts ORDER By CREATED_AT DESC LIMIT $1 OFFSET $2
            "#,
            limit,
            offset
        )
        .fetch_all(&self.pool)
        .await {
            Ok(posts) => {
                Ok(posts)
            },
            Err(e) => {
                Err(ServerError::InternalServerError(Box::new(e)))
            },
        }
    }
}