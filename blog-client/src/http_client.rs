use reqwest::{Client, Response, StatusCode};
use serde_json::json;

pub const TIMEOUT_SEC: u64 = 30; 

use crate::{
    AuthResponse, BlogClientInterface, PATH_BASE, PATH_LOGIN, PATH_POSTS, PATH_PROTECTED, PATH_REGISTER, Post, PostsList, blog_proto::{CreatePostRequest, CreatePostResponse, CreateRegisterRequest, DeletePostRequest, DeletePostResponse, GetPostResponse, ListPostsRequest, ListPostsResponse, LoginRequest, LoginResponse, RegisterResponse, UpdatePostRequest, UpdatePostResponse}, error::ClientError
};

pub struct BlogHttpClient {
    client: Client,
    base_url: String,
}

impl BlogHttpClient {

    pub fn new(url: String) -> Result<Self, ClientError> {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(TIMEOUT_SEC))
            .build()?;
        Ok(Self { client, base_url: url })
    }

    // pub async fn register(&self, req: CreateRegisterRequest) -> Result<AuthResponse, ClientError> {
    //     let res = self.client
    //         .post(format!("{}{}{}", self.base_url, PATH_BASE, PATH_REGISTER))
    //         .json(&json!({"username": req.username, "email": req.email, "password": req.password}))
    //         .send()
    //         .await?;
    //     match self.parse_http_server_response(res).await {
    //         Ok(result) => Ok(result.json().await?),
    //         Err(e) => Err(e)
    //     }
    // }

    // pub async fn login(&self, req: LoginRequest) -> Result<AuthResponse, ClientError> {
    //     let res = self.client
    //         //.post(format!("{}/api/auth/login", self.base_url))
    //         .post(format!("{}{}{}", self.base_url, PATH_BASE, PATH_LOGIN))
    //         .json(&json!({"username": req.email, "password": req.password}))
    //         .send()
    //         .await?;
    //     match self.parse_http_server_response(res).await {
    //         Ok(result) => Ok(result.json().await?),
    //         Err(e) => Err(e)
    //     }
    // }

    // pub async fn new_post(&self, token: &str, req: CreatePostRequest) -> Result<Post, ClientError> {
    //     let res = self.client
    //         //.post(format!("{}/api/posts", self.base_url))
    //         .post(format!("{}{}{}{}", self.base_url, PATH_BASE, PATH_PROTECTED, PATH_POSTS))
    //         .bearer_auth(token)
    //         .json(&json!({"title": req.title, "content": req.content, "author_id": req.author_id}))
    //         .send()
    //         .await?;
    //     match self.parse_http_server_response(res).await {
    //         Ok(result) => Ok(result.json().await?),
    //         Err(e) => Err(e)
    //     }
    // }

    // pub async fn get_post(&self, post_id: i64) -> Result<Post, ClientError> {
    //     let res = self.client
    //         .get(format!("{}{}{}/{}", self.base_url, PATH_BASE, PATH_POSTS, post_id))
    //         .send()
    //         .await?;
    //     match self.parse_http_server_response(res).await {
    //         Ok(result) => Ok(result.json().await?),
    //         Err(e) => Err(e)
    //     }
    // }

    // pub async fn edit_post(&self, token: &str, req: UpdatePostRequest) -> Result<Post, ClientError> {
    //     let res = self.client
    //         .put(format!("{}{}{}{}/{}", self.base_url, PATH_BASE, PATH_PROTECTED, PATH_POSTS, req.post_id))
    //         .bearer_auth(token)
    //         .json(&json!({"title": req.title, "content": req.content, "author_id": req.author_id}))
    //         .send()
    //         .await?;
    //     match self.parse_http_server_response(res).await {
    //         Ok(result) => Ok(result.json().await?),
    //         Err(e) => Err(e)
    //     }
    // }

    // pub async fn del_post(&self, token: &str, id: i64) -> Result<(), ClientError> {
    //     let res = self.client
    //         .delete(format!("{}{}{}{}/{}", self.base_url, PATH_BASE, PATH_PROTECTED, PATH_POSTS, id))
    //         .bearer_auth(token)
    //         .send()
    //         .await?;
    //     match self.parse_http_server_response(res).await {
    //         Ok(_) => Ok(()),
    //         Err(e) => Err(e)
    //     }
    // }

    // pub async fn posts_list(&self, limit: i64, offset: i64) -> Result<PostsList, ClientError> {
    //     let res = self.client
    //         .get(format!("{}{}{}?limit={}&offset={}", self.base_url, PATH_BASE, PATH_POSTS, limit, offset))
    //         .send()
    //         .await?;
    //     match self.parse_http_server_response(res).await {
    //         Ok(result) => Ok(result.json().await?),
    //         Err(e) => Err(e)
    //     }
    // }

    pub async fn parse_http_server_response(&self, res: Response) -> Result<Response, ClientError> {
        if res.status() == StatusCode::CONFLICT {
            return Err(ClientError::UserAlreadyExistsError);
        }
        if res.status() == StatusCode::UNAUTHORIZED {
            return Err(ClientError::UnAuthorizedError);
        }
        if res.status() == StatusCode::FORBIDDEN {
            return Err(ClientError::Forbidden);
        }
        if res.status() == StatusCode::NOT_FOUND {
            return Err(ClientError::ItemNotFoundError);
        }
        if res.status() == StatusCode::BAD_REQUEST {
            let text = res.text().await.unwrap_or_default();
            return Err(ClientError::InvalidRequestError(text));
        }
        if !res.status().is_success() {
            let text = res.text().await.unwrap_or_default();
                return Err(ClientError::ServerError(text));
            }
        Ok(res)
    }
}

impl BlogClientInterface for BlogHttpClient {

    async fn register(&self, req: CreateRegisterRequest) -> Result<RegisterResponse, ClientError> {
        let res = self.client
            .post(format!("{}{}{}", self.base_url, PATH_BASE, PATH_REGISTER))
            .json(&json!({"username": req.username, "email": req.email, "password": req.password}))
            .send()
            .await?;
        match self.parse_http_server_response(res).await {
            Ok(result) => Ok(result.json().await?),
            Err(e) => Err(e)
        }
    }

    async fn login(&self, req: LoginRequest) -> Result<LoginResponse, ClientError> {
        let res = self.client
            .post(format!("{}{}{}", self.base_url, PATH_BASE, PATH_LOGIN))
            .json(&json!({"username": req.email, "password": req.password}))
            .send()
            .await?;
        match self.parse_http_server_response(res).await {
            Ok(result) => Ok(result.json().await?),
            Err(e) => Err(e)
        }
    }

    async fn new_post(&self, token: &str, req: CreatePostRequest) -> Result<CreatePostResponse, ClientError> {
        let res = self.client
            .post(format!("{}{}{}{}", self.base_url, PATH_BASE, PATH_PROTECTED, PATH_POSTS))
            .bearer_auth(token)
            .json(&json!({"title": req.title, "content": req.content, "author_id": req.author_id}))
            .send()
            .await?;
        match self.parse_http_server_response(res).await {
            Ok(result) => Ok(result.json().await?),
            Err(e) => Err(e)
        }
    }

    async fn get_post(&self, post_id: i64) -> Result<GetPostResponse, ClientError> {
        let res = self.client
            .get(format!("{}{}{}/{}", self.base_url, PATH_BASE, PATH_POSTS, post_id))
            .send()
            .await?;
        match self.parse_http_server_response(res).await {
            Ok(result) => Ok(result.json().await?),
            Err(e) => Err(e)
        }
    }

    async fn update_post(&self, token: &str, req: UpdatePostRequest) -> Result<UpdatePostResponse, ClientError> {
        let res = self.client
            .put(format!("{}{}{}{}/{}", self.base_url, PATH_BASE, PATH_PROTECTED, PATH_POSTS, req.post_id))
            .bearer_auth(token)
            .json(&json!({"title": req.title, "content": req.content, "author_id": req.author_id}))
            .send()
            .await?;
        match self.parse_http_server_response(res).await {
            Ok(result) => Ok(result.json().await?),
            Err(e) => Err(e)
        }
    }

    async fn del_post(&self, token: &str, req: DeletePostRequest) -> Result<DeletePostResponse, ClientError> {
        let res = self.client
            .delete(format!("{}{}{}{}/{}", self.base_url, PATH_BASE, PATH_PROTECTED, PATH_POSTS, req.post_id))
            .bearer_auth(token)
            .send()
            .await?;
        match self.parse_http_server_response(res).await {
            Ok(_) => Ok(DeletePostResponse { is_del: true }),
            Err(e) => Err(e)
        }
    }

    async fn posts_list(&self, req: ListPostsRequest) -> Result<ListPostsResponse, ClientError> {
        let res = self.client
            .get(format!("{}{}{}?limit={}&offset={}", self.base_url, PATH_BASE, PATH_POSTS, req.limit, req.offset))
            .send()
            .await?;
        match self.parse_http_server_response(res).await {
            Ok(result) => Ok(result.json().await?),
            Err(e) => Err(e)
        }
    }
}
