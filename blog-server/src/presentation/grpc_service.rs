use std::sync::Arc;

use tonic::{Request, Response, Status};

use crate::{
    application::{auth_service::AuthService, blog_service::BlogService}, 
    blog::{self, CreatePostRequest, CreatePostResponse, CreateRegisterRequest, DeletePostRequest, 
        DeletePostResponse, GetPostRequest, GetPostResponse, ListPostsRequest, ListPostsResponse, 
        LoginRequest, LoginResponse, RegisterResponse, UpdatePostRequest, UpdatePostResponse, UserInfo}, 
    domain::{self, error::ServerError, post::{PostCreatedInfo, PostUpdateInfo}, user::{UserLoginInfo, UserRegisterInfo}}, infrastructure::jwt::JwtService
};

pub struct BlogGrpcService {
    auth_service: Arc<AuthService>,
    blog_service: Arc<BlogService>,
    jwt_service: Arc<JwtService>
}

impl BlogGrpcService {
    pub fn new(
        auth_service: Arc<AuthService>,
        blog_service: Arc<BlogService>,
        jwt_service: Arc<JwtService>
    ) -> Self {
        Self {
            auth_service, blog_service, jwt_service
        }
    }

    pub fn get_user_id_from_metadata(&self, metadata: &tonic::metadata::MetadataMap) -> Result<i64, Status> {
        let token = metadata
            .get("authorization")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.strip_prefix("Bearer "))
            .ok_or_else(|| Status::unauthenticated("Missing authorization token"))?;

        let claims = self.jwt_service.verify_token(token)
            .map_err(|_| Status::unauthenticated("Invalid token"))?;

        Ok(claims.user_id)
    }

    pub fn transform_post_to_grpc_f(&self, base_post: &domain::post::Post) -> blog::Post {
        blog::Post {
            id: base_post.id,
            title: base_post.title.clone(),
            content: base_post.content.clone(),
            author_id: base_post.author_id,
        }
    }
}

#[tonic::async_trait]
impl blog::blog_service_server::BlogService for BlogGrpcService {
    
    async fn register(
        &self, 
        request: Request<CreateRegisterRequest>
    ) -> Result<Response<RegisterResponse>, Status> {

        let req = request.into_inner();

        let (token, user) = self.auth_service.create_user(
            UserRegisterInfo { username: req.username , email: req.email, password: req.password }
        ).await
        .map_err(|e| match e {
            ServerError::UserAlreadyExistsError => {
                Status::already_exists("User already exists")
            },
            other => Status::internal(other.to_string()),
        })?;

        let reply = RegisterResponse { 
            token, 
            user: Some(UserInfo { 
                username: user.username, 
                email: user.email
            }) 
        };
        Ok(Response::new(reply))
    }


    async fn login(
        &self, 
        request: Request<LoginRequest>
    ) -> Result<Response<LoginResponse>, Status> {

        let req = request.into_inner();

        let (token, user) = self.auth_service.login_user(
            UserLoginInfo { email: req.email, password: req.password }
        ).await
        .map_err(|e| match e {
            ServerError::UserNotFoundError => {
                Status::not_found("Invalid login or password")
            },
            other => Status::internal(other.to_string()),
        })?;

        let reply = LoginResponse {
            token,
            user: Some(UserInfo{username:user.username,email:user.email})
        };
        Ok(Response::new(reply))
    }


    // CRUD
    async fn create_post(
        &self, 
        request: Request<CreatePostRequest>
    ) -> Result<Response<CreatePostResponse>, Status> {


        let jwt_user_id = self.get_user_id_from_metadata(request.metadata())?;
        let req = request.into_inner();

        let post = self.blog_service.new_post(
            PostCreatedInfo {
                author_id: jwt_user_id,
                title: req.title, 
                content: req.content
            }
        ).await
        .map_err(|e| match e {
            ServerError::PostNotFoundError => {
                Status::not_found("Post not found")
            },
            ServerError::Forbidden => {
                Status::unauthenticated("Permission denied")
            },
            other => Status::internal(other.to_string()),
        })?;

        let grpc_post = self.transform_post_to_grpc_f(&post);
        let reply = CreatePostResponse { post: Some(grpc_post) };
        Ok(Response::new(reply))
    }


    async fn get_post(&self,
        request: Request<GetPostRequest>
    ) -> Result<Response<GetPostResponse>, Status> {

        let req = request.into_inner();

        let post = 
        self.blog_service.get_post(req.post_id).await
        .map_err(|e| match e {
            ServerError::PostNotFoundError => {
                Status::not_found("Post not found")
            },
            other => Status::internal(other.to_string()),
        })?;

        let grpc_post = self.transform_post_to_grpc_f(&post);
        let reply = GetPostResponse { post: Some(grpc_post) };
        
        Ok(Response::new(reply))
    }


    async fn update_post(
        &self,
        request: Request<UpdatePostRequest>
    ) -> Result<Response<UpdatePostResponse>, Status> {

        let jwt_user_id = self.get_user_id_from_metadata(request.metadata())?;
        let req = request.into_inner();

        let post = self.blog_service.edit_post(
            jwt_user_id, 
            req.post_id, 
            PostUpdateInfo { 
                title: req.title, 
                content: req.content
            }
        ).await
        .map_err(|e| match e {
            ServerError::PostNotFoundError => {
                Status::not_found("Post not found")
            },
            ServerError::Forbidden => {
                Status::unauthenticated("Permission denied")
            },
            other => Status::internal(other.to_string()),
        })?;

        let grpc_post = self.transform_post_to_grpc_f(&post);
        let reply = UpdatePostResponse { post: Some(grpc_post) };
        
        Ok(Response::new(reply))
    }
    

    async fn delete_post(
        &self, 
        request: Request<DeletePostRequest>
    ) -> Result<Response<DeletePostResponse>, Status> {

        let jwt_user_id = self.get_user_id_from_metadata(request.metadata())?;
        let req = request.into_inner();

        let result 
        = self.blog_service.del_post(jwt_user_id, req.post_id).await
        .map_err(|e| match e {
            ServerError::PostNotFoundError => {
                Status::not_found("Post not found")
            },
            ServerError::Forbidden => {
                Status::unauthenticated("Permission denied")
            },
            other => Status::internal(other.to_string()),
        })?;

        let reply = 
            DeletePostResponse { is_del: result};
        Ok(Response::new(reply))
    }

    async fn list_posts(
        &self,
        request: Request<ListPostsRequest>
    ) -> Result<Response<ListPostsResponse>, Status> {

        let req = request.into_inner();
        let offset = req.offset;
        let limit = req.limit;

        let post_list = self.blog_service.posts_list(offset, limit).await
        .map_err(|e| match e {
            ServerError::PaganationError => Status::invalid_argument(e.to_string()),
            other => Status::internal(other.to_string()),
        })?;

        let grpc_post_list: Vec<blog::Post> = 
            post_list.iter()
            .map(|p| self.transform_post_to_grpc_f(p))
            .collect();

        let reply = 
            ListPostsResponse { posts: grpc_post_list };
        Ok(Response::new(reply))
    }
    
}