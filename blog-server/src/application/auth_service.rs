use std::sync::Arc;

use argon2::{
    Algorithm, Argon2, Params, PasswordVerifier, Version, 
    password_hash::{PasswordHash, PasswordHasher, SaltString, rand_core::OsRng}
};

use crate::{data::user_repository::UserRepository, domain::{error::ServerError, user::{User, UserLoginInfo, UserRegisterInfo}}, infrastructure::jwt::JwtService};

pub struct AuthService {
    user_store: Arc<UserRepository>,
    jwt_service: Arc<JwtService>,
}

impl AuthService {

    pub fn new(
        user_store: Arc<UserRepository>,
        jwt_service: Arc<JwtService>,
    ) -> Self {
        Self { user_store, jwt_service }
    }

    pub async fn create_user(&self, user: UserRegisterInfo) -> Result<(String, User), ServerError> {

        tracing::info!("Creating user from data: {}, {}", user.username, user.email);
        let origin_pass = user.password.clone();

        let salt = SaltString::generate(&mut OsRng);
        let params = Params::new(19 * 1024, 2, 1, None)?;
        let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);
        let hash_pass = argon2.hash_password(origin_pass.as_bytes(), &salt)?;

        let new_user = self.user_store.insert_user(
                user.username.clone(), user.email.clone(), hash_pass.to_string()
        ).await?;

        let token = self.jwt_service.generate_token(
            new_user.id, new_user.username.as_str()
        )?;

        tracing::info!("New user is created and sending");
        Ok((token, new_user))
    }

    pub async fn login_user(&self, user: UserLoginInfo) -> Result<(String, User), ServerError> {

        tracing::info!("Attemp to login from {}", user.email);

        let logged_user = self.user_store.get_user_by_email(
            user.email.clone()
        ).await?;

        let parsed_hash = PasswordHash::new(&logged_user.password_hash.as_str())?;
        let argon2 = Argon2::default();
        let verify = argon2.verify_password(user.password.as_bytes(), &parsed_hash).is_ok();

        if !verify {
            tracing::error!("Token from user {} is incorrect", user.email);
            return Err(ServerError::UserNotFoundError)
        }

        let token = self.jwt_service.generate_token(
            logged_user.id, logged_user.username.as_str()
        )?;

        tracing::info!("Login is ok");
        Ok((token, logged_user))        
    }
}