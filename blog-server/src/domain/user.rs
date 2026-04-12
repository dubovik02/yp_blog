use std::fmt;

use chrono::Utc;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct User {
    pub id: i64, 
    pub username: String, 
    pub email: String, 
    pub password_hash: String, 
    pub created_at: chrono::DateTime<Utc>
}

impl User {
    pub fn new() -> Self {
        Self {
            id: 0, 
            username: String::new(), 
            email: String::new(), 
            password_hash: String::new(),
            created_at:  Utc::now()
        }
    }

    pub fn to_string(&self) -> String {
        format!("{} ({})", self.username, self.email)
    }
}

impl Default for User {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", format!("{} ({})", self.username, self.email))
    }
}


#[derive(Serialize, Deserialize, Debug)]
pub struct UserRegisterInfo {
    pub username: String, 
    pub email: String, 
    pub password: String
}

impl fmt::Display for UserRegisterInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.username)
    }
}


#[derive(Serialize, Deserialize, Debug)]
pub struct UserLoginInfo {
    pub email: String, 
    pub password: String
}

impl fmt::Display for UserLoginInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.email)
    }
}


