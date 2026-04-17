use std::sync::Arc;

use actix_web::{HttpResponse, Responder, http::StatusCode, web};

use crate::{
    application::{auth_service::AuthService, blog_service::BlogService}, 
    blog::CreateRegisterRequest, 
    domain::{self, error::ServerError, post::{PaginationQuery, PostCreatedInfo, PostUpdateInfo}, 
    user::{UserLoginInfo, UserRegisterInfo}}, 
    presentation::{http_handlers::blog::{CreatePostRequest, LoginRequest}, 
    middleware::AuthenticatedUser}};

mod blog {
    tonic::include_proto!("blog");
}

pub async fn health() -> impl Responder {
    tracing::info!("Health query is sending");
    HttpResponse::Ok().json(
        serde_json::json!({ "status": "ok" })
    )
}

pub async fn create_user(auth_service: web::Data<Arc<AuthService>>, user: web::Json<CreateRegisterRequest>) -> Result<HttpResponse, ServerError> {

    tracing::info!("Creating user from data: {}, {}", user.username, user.email);
    let user_data = auth_service.create_user(UserRegisterInfo { 
        username: user.username.clone(), 
        email: user.email.clone(), 
        password: user.password.clone() 
    }).await?;

    tracing::info!("New user is created and sending");
    Ok(HttpResponse::Created().json(serde_json::json!(
                {
                    "token": user_data.0,
                    "user": {
                        "username": user_data.1.username,
                        "email": user_data.1.email,
                    }
                }
            )
        )
    )
}

pub async fn login_user(auth_service: web::Data<Arc<AuthService>>, user: web::Json<LoginRequest>) -> Result<HttpResponse, ServerError> {

    tracing::info!("Attemp to login from {}", user.email);

    let logged_user = auth_service.login_user(
        UserLoginInfo { email: user.email.clone(), password: user.password.clone() }
    ).await?;

    tracing::info!("Login is ok");
    Ok(HttpResponse::Ok().json(serde_json::json!(
                {
                    "token": logged_user.0,
                    "user": {
                        "username": logged_user.1.username,
                        "email": logged_user.1.email,
                    }
                }
            )
        )
    )
}

pub async fn new_post(blog_service: web::Data<Arc<BlogService>>, user: AuthenticatedUser, post_data: web::Json<CreatePostRequest>) -> Result<HttpResponse, ServerError> {

    tracing::info!("Attempt to create new post");
    
    let new_post: crate::domain::post::Post = blog_service.new_post(
        PostCreatedInfo {
            author_id: user.user_id,
            title: post_data.title.clone(),
            content: post_data.content.clone()
        }
    ).await?;

    tracing::info!("Post created is ok");
    Ok(HttpResponse::Created().json(serde_json::json!({
        "post": convert_domain_post_to_proto_post(new_post)
    })))
}

pub async fn get_post(blog_service: web::Data<Arc<BlogService>>, path: web::Path<i64>) -> Result<HttpResponse, ServerError> {

    tracing::info!("Request for post ID: {}", path.clone());
    let post = blog_service.get_post(
        path.into_inner()
    ).await?;

    tracing::info!("Request for post ok");
    Ok(HttpResponse::Created().json(serde_json::json!({
        "post": convert_domain_post_to_proto_post(post)
    })))
}

pub async fn edit_post(blog_service: web::Data<Arc<BlogService>>, user: AuthenticatedUser, path: web::Path<i64>, 
    post_data: web::Json<PostUpdateInfo>) -> Result<HttpResponse, ServerError> {

    tracing::info!("Request for change post ID: {}", path.clone());
    let upd_post = blog_service.edit_post(
        user.user_id,
        path.into_inner(),
        PostUpdateInfo {
            title: post_data.title.clone(),
            content: post_data.content.clone(),
        }
    ).await?;

    tracing::info!("Post has updated and sending");
    Ok(HttpResponse::Created().json(serde_json::json!({
        "post": convert_domain_post_to_proto_post(upd_post)
    })))
}

pub async fn del_post(blog_service: web::Data<Arc<BlogService>>, user: AuthenticatedUser, path: web::Path<i64>) 
    -> Result<HttpResponse, ServerError> {

    tracing::info!("Request for delete post ID: {}", path.clone());
    let is_post_del = blog_service.del_post(
        user.user_id,
        path.into_inner(),
    ).await?;

    if is_post_del { 
        tracing::info!("Post has deleted");
        return Ok(HttpResponse::new (StatusCode::NO_CONTENT));
    }
    else {
        tracing::warn!("Post has not deleted");
        return Err(ServerError::PostNotFoundError);
    }
}

pub async fn posts_list(blog_service: web::Data<Arc<BlogService>>, query: web::Query<PaginationQuery>) -> Result<HttpResponse, ServerError> {

    let offset = query.offset.unwrap_or(1).into();
    let limit = query.limit.unwrap_or(10).into();

    tracing::info!("Someone asked for post's list with offset = {} and limit = {}", offset, limit);

    let posts = blog_service.posts_list(offset, limit).await?;

    Ok(HttpResponse::Ok().json(serde_json::json!(
        {
            "posts": posts,
            "total": posts.len(),
            "limit": limit,
            "offset:": offset 
        }
    )))

}

fn convert_domain_post_to_proto_post(base_post: domain::post::Post) -> blog::Post {

    blog::Post {
        id: base_post.id,
        title: base_post.title,
        content: base_post.content,
        author_id: base_post.author_id,
    }
}