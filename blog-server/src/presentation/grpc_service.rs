use tonic::{Request, Response, Status};

use crate::{blog::{CreatePostRequest, CreatePostResponse, CreateRegisterRequest, DeletePostRequest, DeletePostResponse, GetPostRequest, GetPostResponse, ListPostsRequest, ListPostsResponse, LoginRequest, LoginResponse, Post, RegisterResponse, UpdatePostRequest, UpdatePostResponse, blog_service_server::BlogService}, 
domain::user::UserRegisterInfo};

pub struct BlogGrpcService {

}

impl BlogGrpcService {
    pub fn new() -> Self {
        Self {
        }
    }
}

#[tonic::async_trait]
impl BlogService for BlogGrpcService {
    
    async fn register(
        &self, 
        request: Request<CreateRegisterRequest>
    ) -> Result<Response<RegisterResponse>, Status> {
        let reply = 
            RegisterResponse { user_id: 1, email: "pass".to_owned() };
        Ok(Response::new(reply))
    }

    async fn login(
        &self, 
        request: Request<LoginRequest>
    ) -> Result<Response<LoginResponse>, Status> {
        let reply = 
            LoginResponse { email: "qwe@qwe".to_owned(), password: "pass".to_owned() };
        Ok(Response::new(reply))
    }

    // CRUD
    async fn create_post(
        &self, 
        request: Request<CreatePostRequest>
    ) -> Result<Response<CreatePostResponse>, Status> {
        let post = Post { 
            id: 0,
            title: "title".to_owned(),
            author_id: 0,
            content: "content".to_owned()
        };
        let reply = 
            CreatePostResponse { post: Some(post) };
        Ok(Response::new(reply))
    }

    async fn get_post(&self,
        request: Request<GetPostRequest>
    ) -> Result<Response<GetPostResponse>, Status> {
         let post = Post { 
            id: 0,
            title: "title".to_owned(),
            author_id: 0,
            content: "content".to_owned()
        };
        let reply = 
            GetPostResponse { post: Some(post) };
        Ok(Response::new(reply))
    }

    async fn update_post(
        &self,
        request: Request<UpdatePostRequest>
    ) -> Result<Response<UpdatePostResponse>, Status> {
        let post = Post { 
            id: 0,
            title: "title".to_owned(),
            author_id: 0,
            content: "content".to_owned()
        };
        let reply = 
            UpdatePostResponse { post: Some(post) };
        Ok(Response::new(reply))
    }
    
    async fn delete_post(
        &self, 
        request: Request<DeletePostRequest>
    ) -> Result<Response<DeletePostResponse>, Status> {
        let reply = 
            DeletePostResponse { post_id: 0 };
        Ok(Response::new(reply))
    }

    async fn list_posts(
        &self,
        request: Request<ListPostsRequest>
    ) -> Result<Response<ListPostsResponse>, Status> {
        let post = Post { 
            id: 0,
            title: "title".to_owned(),
            author_id: 0,
            content: "content".to_owned()
        };
        let post2 = Post { 
            id: 0,
            title: "title".to_owned(),
            author_id: 0,
            content: "content".to_owned()
        };
        let reply = 
            ListPostsResponse { posts: vec![post, post2] };
        Ok(Response::new(reply))
    }
    
}