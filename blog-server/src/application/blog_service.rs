use std::sync::Arc;

use crate::{data::blog_repository::BlogRepository, 
    domain::{error::ServerError, 
    post::{Post, PostCreatedInfo, PostDeleteInfo, PostUpdateInfo}}};

pub struct BlogService {
    blog_store: Arc<BlogRepository>
}

impl BlogService {

    pub fn new(
        blog_store: Arc<BlogRepository>,
    ) -> Self {
        Self { blog_store }
    }

    pub async fn new_post(&self, post_data: PostCreatedInfo) -> Result<Post, ServerError> {

        let new_post = self.blog_store.insert_post(
            PostCreatedInfo {
                author_id: post_data.author_id,
                title: post_data.title.clone(),
                content: post_data.content.clone()
            }
        ).await?;

        Ok(new_post)
    }

    pub async fn get_post(&self, post_id: i64) -> Result<Post, ServerError> {

        let post = self.blog_store.get_post_by_id(
            post_id
        ).await?;

        Ok(post)
    }

    pub async fn edit_post(&self, user_id: i64, post_id: i64, 
        post_data: PostUpdateInfo) -> Result<Post, ServerError> {

        let post = self.blog_store.get_post_by_id(post_id).await?;

        if post.author_id != user_id { 
            return Err(ServerError::Forbidden)
        };

        let upd_post = self.blog_store.update_post_by_id(
            post_id,
            PostUpdateInfo {
                title: post_data.title.clone(),
                content: post_data.content.clone()
            }
        ).await?;
        Ok(upd_post)
    }

    pub async fn del_post(&self, user_id: i64, post_id: i64) -> Result<bool, ServerError> {

        let post = self.blog_store.get_post_by_id(post_id).await?;

        if post.author_id != user_id { 
            return Err(ServerError::Forbidden)
        };

        let is_post_del = self.blog_store.delete_post_by_id(
            PostDeleteInfo {
                id: post_id,
                author_id: user_id,
            }
        ).await?;

        if is_post_del { 
            return Ok(true);
        }
        else {
            return Err(ServerError::PostNotFoundError);
        }
    }

    pub async fn posts_list(&self, offset: i64, limit: i64) -> Result<Vec<Post>, ServerError> {

        if offset < 0 || offset > 100 { return Err(ServerError::PaganationError) }
        if limit < 0 || limit > 100 { return Err(ServerError::PaganationError) }

        let posts = self.blog_store.get_posts_list(offset, limit).await?;
        Ok(posts)

    }
}