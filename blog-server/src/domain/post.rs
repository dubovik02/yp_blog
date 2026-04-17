use chrono::Utc;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct Post {
    pub id: i64, 
    pub title: String, 
    pub content: String, 
    pub author_id: i64, 
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PostCreatedInfo {
    pub author_id: i64,
    pub title: String, 
    pub content: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PostUpdateInfo {
    pub title: String, 
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PostDeleteInfo {
    pub id: i64, 
    pub author_id: i64
}

#[derive(Deserialize)]
pub struct PaginationQuery {
    pub offset: Option<u32>,
    pub limit: Option<u32>,
}

impl Default for Post {
    fn default() -> Self {
        Self::new()
    }
}

impl Post {
    pub fn new() -> Self {
        Self {
            id: 0,
            title: String::new(), 
            content: String::new(),
            author_id: 0,
            created_at:  Utc::now(),
            updated_at:  Utc::now()
        }
    }
}