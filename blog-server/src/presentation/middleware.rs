use std::{env, future::{Ready, ready}};

use actix_web::{FromRequest, HttpMessage, HttpRequest, dev::ServiceRequest, error::ErrorUnauthorized};
use actix_web_httpauth::extractors::bearer::BearerAuth;

use crate::{JWT_SECRET_KEY, infrastructure::jwt::JwtService};

#[derive(Debug, Clone)]
pub struct  AuthenticatedUser {
    pub user_id: i64,
    pub user_name: String
}

impl FromRequest for AuthenticatedUser {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, payload: &mut actix_web::dev::Payload) -> Self::Future {
        let value = req.extensions().get::<AuthenticatedUser>().cloned();
        match value {
            Some(user) => ready(Ok(user)),
            None => ready(Err(ErrorUnauthorized("No claims"))),
        }
    }
}

pub async fn jwt_validator(req: ServiceRequest, token: BearerAuth) -> Result<ServiceRequest, (actix_web::Error, ServiceRequest)> {

    let secret_key = env::var("JWT_SECRET")
        .unwrap_or_else(|_| JWT_SECRET_KEY.to_string());
    let jwt_service = JwtService::new(&secret_key);
    let verify = jwt_service.verify_token(token.token());

    match verify {
        Ok(claims) => {
            req.extensions_mut().insert(AuthenticatedUser {
                user_id: claims.user_id,
                user_name: claims.username
            });
            Ok(req)
        }
        Err(_) => Err((actix_web::error::ErrorUnauthorized("Invalid token"), req))
    }

}

