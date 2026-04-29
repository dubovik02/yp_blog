use std::sync::{Arc};

use chrono::Utc;
use serde::{Deserialize, Serialize};

use tokio::sync::Mutex;

use crate::{blog_proto::{CreatePostRequest, CreatePostResponse, CreateRegisterRequest, DeletePostRequest, 
    DeletePostResponse, GetPostResponse, ListPostsRequest, ListPostsResponse, LoginRequest, 
    LoginResponse, RegisterResponse, UpdatePostRequest, UpdatePostResponse}, error::ClientError, 
    grpc_client::BlogGrpcClient, http_client::BlogHttpClient};

pub mod http_client;
pub mod grpc_client;
pub mod error;
pub mod blog_proto {
    tonic::include_proto!("blog");
}

pub const PATH_BASE: &str = "/api/v1";
pub const PATH_PROTECTED: &str = "/protected";
pub const PATH_REGISTER: &str = "/auth/register";
pub const PATH_LOGIN: &str = "/auth/login";
pub const PATH_POSTS: &str = "/posts";


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientUserInfo {
    pub username: String,
    pub email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: ClientUserInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    pub id: i64, 
    pub title: String, 
    pub content: String, 
    pub author_id: i64, 
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostsList {
    pub posts: Vec<Post>,
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
}

#[derive(Debug, Clone)]
pub enum Transport {
    Http,
    Grpc,
}

pub struct BlogClient {
    transport: Transport,
    token: Arc<Mutex<Option<String>>>,
    http_client: Arc<BlogHttpClient>,
    grpc_client: Arc<BlogGrpcClient>,
}

impl BlogClient {

    pub async fn new(url: String, transport: Transport) -> Result<Self, ClientError> {
        Ok(Self {  
            transport,
            token: Arc::new(Mutex::new(None)),
            http_client: Arc::new(BlogHttpClient::new(url.clone())?),
            grpc_client: Arc::new(BlogGrpcClient::new(url.clone())),
        })
    }

    pub async fn set_token(&self, token: String) {
        *self.token.lock().await = Some(token);
    }

    pub async fn get_token(&self) -> Option<String> {
        self.token.lock().await.clone()
    }

    pub async fn register(&self, username: &str, email: &str, password: &str) -> Result<RegisterResponse, ClientError> {

        let req = CreateRegisterRequest {
            username: username.to_string(),
            email: email.to_string(),
            password: password.to_string()
        };
        let res = match &self.transport {
            Transport::Http => self.http_client.register(req).await?,
            Transport::Grpc => self.grpc_client.register(req).await?,
        };
        self.set_token(res.token.clone()).await;
        Ok(res)
    }

    pub async fn login(&self, email: &str, password: &str) -> Result<LoginResponse, ClientError> {

        let req = LoginRequest {
            email: email.to_string(),
            password: password.to_string()
        };
        let res = match &self.transport {
            Transport::Http => self.http_client.login(req).await?,
            Transport::Grpc => self.grpc_client.login(req).await?,
        };
        self.set_token(res.token.clone()).await;
        Ok(res)
    }

    pub async fn new_post(&self, title: &str, content: &str) -> Result<CreatePostResponse, ClientError> {
        let token = self.token.lock().await.clone()
            .ok_or(ClientError::UnAuthorizedError)?;

        let req = CreatePostRequest {
            title: title.to_string(),
            content: content.to_string(),
        };
        let res = match &self.transport {
            Transport::Http => self.http_client.new_post(&token, req).await?,
            Transport::Grpc => self.grpc_client.new_post(&token, req).await?,
        };
        Ok(res)
    }

    pub async fn get_post(&self, id: i64) -> Result<GetPostResponse, ClientError> {
        let res = match &self.transport {
            Transport::Http => self.http_client.get_post(id).await?,
            Transport::Grpc => self.grpc_client.get_post(id).await?,
        };
        Ok(res)
    }

    pub async fn update_post(&self, id: i64, title: &str, content: &str) -> Result<UpdatePostResponse, ClientError> {

        let token = self.token.lock().await.clone()
            .ok_or(ClientError::UnAuthorizedError)?;

        let req = UpdatePostRequest {
            post_id: id,
            title: title.to_string(),
            content: content.to_string(),
        };
        let res = match &self.transport {
            Transport::Http => self.http_client.update_post(&token, req).await?,
            Transport::Grpc => self.grpc_client.update_post(&token, req).await?,
        };
        Ok(res)
    }

    pub async fn del_post(&self, id: i64) -> Result<DeletePostResponse, ClientError> {
        let token = self.token.lock().await.clone()
            .ok_or(ClientError::UnAuthorizedError)?;

        let req = DeletePostRequest { post_id: id };

        let res = match &self.transport {
            Transport::Http => self.http_client.del_post(&token, req).await?,
            Transport::Grpc => self.grpc_client.del_post(&token, req).await?,
        };
        Ok(res)

    }

    pub async fn list_posts(&self, limit: i64, offset: i64) -> Result<ListPostsResponse, ClientError> {

        let req = ListPostsRequest { offset, limit };

        let res = match &self.transport {
            Transport::Http => self.http_client.posts_list(req).await?,
            Transport::Grpc => self.grpc_client.posts_list(req).await?,
        };
        Ok(res)
    }
}
