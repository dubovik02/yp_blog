use std::{env, sync::Arc};

use actix_cors::Cors;
use actix_web::{App, HttpServer, middleware::{self, DefaultHeaders}, web};
use actix_web_httpauth::middleware::HttpAuthentication;
use dotenvy::dotenv;
use tonic::transport::Server;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

use crate::{application::{auth_service::AuthService, blog_service::BlogService}, blog::blog_service_server::BlogServiceServer, data::{blog_repository::BlogRepository, user_repository::UserRepository}, domain::error::ServerError, infrastructure::{database::{create_pool, create_schema}, jwt::JwtService}, presentation::{grpc_service::BlogGrpcService, http_handlers::{create_user, del_post, edit_post, get_post, health, login_user, new_post, posts_list}, middleware::jwt_validator}};

mod domain;
mod infrastructure;
mod presentation;
mod application;
mod data;

mod blog {
    tonic::include_proto!("blog");
}

const SERVER_HOST: &str = "127.0.0.1"; 
const SERVER_PORT: &str = "8080"; 

const GRPC_SERVER_HOST: &str = "127.0.0.1"; 
const GRPC_SERVER_PORT: &str = "50051"; 

const JWT_SECRET_KEY: &str = "secret_key";

#[actix_web::main]
async fn main() -> Result<(), ServerError>{

    init_logging();
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://postgres:qwerty@localhost:5432/postgres".to_string());

    tracing::info!("connect to DB {}", database_url);

    let pool = create_pool(&database_url).await?;

    match (create_schema(&pool)).await {
        Ok(_) => {println!("DB schema has created.");},
        Err(e) => {println!("Error while creating schema: {}", e);}
    }

    let http_addr = format!("{}:{}", SERVER_HOST, SERVER_PORT); 

    let jwt_service = Arc::new(JwtService::new(JWT_SECRET_KEY));
    let user_store = Arc::new(UserRepository::new(pool.clone()));
    let auth_service = Arc::new(AuthService::new(user_store.clone(), jwt_service.clone()));
    let blog_repository = Arc::new(BlogRepository::new(pool.clone()));
    let blog_service = Arc::new(BlogService::new(blog_repository.clone()));

    let http_auth_service = auth_service.clone();
    let http_blog_service = blog_service.clone();
    
    println!("HTTP server has started on {}", http_addr);

    let http = HttpServer::new(move || {

        let cors = Cors::default()
        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
        .allowed_headers(vec![
            actix_web::http::header::CONTENT_TYPE,
            actix_web::http::header::AUTHORIZATION,
        ])
        .supports_credentials()
        .max_age(3600);
    
        App::new()
        .app_data(web::Data::new(http_auth_service.clone()))
        .app_data(web::Data::new(http_blog_service.clone()))
        .wrap(DefaultHeaders::new()
                .add(("X-Content-Type-Options", "nosniff"))
                .add(("Referrer-Policy", "no-referrer"))
                .add(("Permissions-Policy", "geolocation=()"))
                .add(("Cross-Origin-Opener-Policy", "same-origin")))
        .wrap(cors)
        .service(web::scope("/api/v1/protected")
            .wrap(HttpAuthentication::bearer(jwt_validator))
            .route("/posts/new", web::post().to(new_post))
            .route("/posts/{id}", web::put().to(edit_post))
            .route("/posts/{id}", web::delete().to(del_post))
        )
        .service(web::scope("/api/v1")
            .route("/health", web::get().to(health))
            .route("/auth/register", web::post().to(create_user))
            .route("/auth/login", web::post().to(login_user))
            .route("/posts/{id}", web::get().to(get_post))
            .route("/posts", web::get().to(posts_list))
        )
            .wrap(middleware::Logger::default())
        })
        .bind(http_addr)?
        .run();

    let grpc_addr = format!("{}:{}", GRPC_SERVER_HOST, GRPC_SERVER_PORT).parse()?;
    let blog_service = BlogGrpcService::new();

    println!("gRPC server has started on {}", grpc_addr);
    let grpc = Server::builder()
        .add_service(BlogServiceServer::new(blog_service))
        .serve(grpc_addr);

    tokio::select! {
        res = http => {
            println!("HTTP-server process has finished: {:?}", res);
        }
        res = grpc => {
            println!("gRPC-server process has finished: {:?}", res);
        }
    }

    Ok(())
}

fn init_logging() {
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "actix_web=info,my_app=debug".into()))
        .with(
            tracing_subscriber::fmt::layer()
                .with_target(false)
                .with_timer(tracing_subscriber::fmt::time::ChronoUtc::rfc_3339())
        )
        .init();
    
    tracing::info!("Logging has been initialized");
}
