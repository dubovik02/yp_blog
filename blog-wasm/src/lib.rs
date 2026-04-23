use gloo_net::http::{Request, Response};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use web_sys::window;


const TOKEN_LOCAL_STORAGE_KEY: &str = "blog_token";

pub const BASE_URL: &str = "http://localhost:3000";
pub const PATH_BASE: &str = "/api/v1";
pub const PATH_PROTECTED: &str = "/protected";
pub const PATH_REGISTER: &str = "/auth/register";
pub const PATH_LOGIN: &str = "/auth/login";
pub const PATH_POSTS: &str = "/posts";
pub const PATH_POSTS_NEW: &str = "/new";
pub const PATH_ME: &str = "/me";

#[derive(Serialize, Deserialize, Debug)]
pub struct UserRegisterInfo {
    pub username: String, 
    pub email: String, 
    pub password: String
}

#[derive(Serialize, Deserialize)]
struct UserInfo {
    id: i64,
    username: String,
    email: String,
}

#[derive(Serialize, Deserialize)]
struct AuthResponse {
    token: String,
    user: UserInfo,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct WasmPost {
    pub id: i64,
    pub title: String,
    pub content: String,
    pub author_id: i64
}

#[derive(Serialize, Deserialize, Clone)]
pub struct WasmPostResponse {
    pub post: WasmPost
}

#[derive(Serialize, Deserialize)]
struct PostsList {
    posts: Vec<WasmPost>,
    total: i64,
    limit: i64,
    offset: i64,
}

#[derive(Serialize, Deserialize)]
struct ErrorGlooResponse {
    error_text: String,
}

#[wasm_bindgen]
pub struct BlogApp {
}

#[wasm_bindgen]
pub fn save_token_to_storage(token: &str) {
    // if let Some(window) = window() {
    //     if let Ok(Some(storage)) = window.local_storage() {
    //         let _ = storage.set_item(TOKEN_LOCAL_STORAGE_KEY, token);
    //     }
    // }
    if let Some(window) = window()
         && let Ok(Some(storage)) = window.local_storage() {
             let _ = storage.set_item(TOKEN_LOCAL_STORAGE_KEY, token);
    }
}

#[wasm_bindgen]
pub fn get_token_from_storage() -> Option<String> {
    let window = window()?;
    let storage = window.local_storage().ok()??;
    storage.get_item(TOKEN_LOCAL_STORAGE_KEY).ok()?
}

#[wasm_bindgen]
pub fn del_token() {
    // if let Some(window) = window() {
    //     if let Ok(Some(storage)) = window.local_storage() {
    //         let _ = storage.remove_item(TOKEN_LOCAL_STORAGE_KEY);
    //     }
    // }
    if let Some(window) = window()
         && let Ok(Some(storage)) = window.local_storage() {
             let _ = storage.remove_item(TOKEN_LOCAL_STORAGE_KEY);
    }
}

fn check_error<T>(err_msg: &str) -> Result<T, JsValue> {
    Err(JsValue::from_str(err_msg))
}

fn check_network_error(error: impl std::fmt::Display) -> JsValue {
    let err_msg = 
        format!("Server {} has not avaliable. Check connection or server address. (Err: {})", BASE_URL, error);
    JsValue::from_str(&err_msg)
}

async fn get_error_msg(res: Response, fallback: &str) -> String {
    let status = res.status();
    let text = res.text().await.unwrap_or_default();

    if let Ok(parsed) = serde_json::from_str::<ErrorGlooResponse>(&text) {
        return parsed.error_text;
    }
    if !text.trim().is_empty() {
        return text;
    }

    format!("{fallback} (HTTP {status})")
}

#[wasm_bindgen]
pub async fn register(username: String, email: String, password: String) -> Result<JsValue, JsValue> {
    if username.trim().is_empty() || email.trim().is_empty() || password.is_empty() {
        return check_error("You have to set username, email, password.");
    }

    let body = serde_json::json!({"username": username, "email": email, "password": password});
    let res = Request::post(&format!("{}{}{}", BASE_URL, PATH_BASE, PATH_REGISTER))
        .header("Content-Type", "application/json")
        .body(body.to_string())
        .map_err(|e| JsValue::from_str(&e.to_string()))?
        .send()
        .await
        .map_err(check_network_error)?;

    if !res.ok() {
        let text = get_error_msg(res, "Error while registering").await;
        return Err(JsValue::from_str(&text));
    }

    let auth: AuthResponse = res.json().await.map_err(|e| JsValue::from_str(&e.to_string()))?;
    save_token_to_storage(&auth.token);
    serde_wasm_bindgen::to_value(&auth).map_err(|e| JsValue::from_str(&e.to_string()))
}

#[wasm_bindgen]
pub async fn login(email: String, password: String) -> Result<JsValue, JsValue> {
    if email.trim().is_empty() || password.is_empty() {
        return check_error("You have to set email and password.");
    }

    let body = serde_json::json!({"email": email, "password": password});
    let res = Request::post(&format!("{}{}{}", BASE_URL, PATH_BASE, PATH_LOGIN))
        .header("Content-Type", "application/json")
        .body(body.to_string())
        .map_err(|e| JsValue::from_str(&e.to_string()))?
        .send()
        .await
        .map_err(check_network_error)?;

    if !res.ok() {
        let text = get_error_msg(res, "Error while sign in.").await;
        return Err(JsValue::from_str(&text));
    }

    let auth: AuthResponse = res.json().await.map_err(|e| JsValue::from_str(&e.to_string()))?;
    save_token_to_storage(&auth.token);
    serde_wasm_bindgen::to_value(&auth).map_err(|e| JsValue::from_str(&e.to_string()))
}

#[wasm_bindgen]
pub async fn get_user_info() -> Result<JsValue, JsValue> {

    let token = get_token_from_storage().ok_or_else(|| JsValue::from_str("You have to be authorized"))?;
    let res = Request::get(&format!("{}{}{}{}", BASE_URL, PATH_BASE, PATH_PROTECTED, PATH_ME))
        .header("Content-Type", "application/json")
        .header("Authorization", &format!("Bearer {token}"))
        .send()
        .await
        .map_err(check_network_error)?;

    if !res.ok() {
        let text = get_error_msg(res, "Error while getting user info").await;
        return Err(JsValue::from_str(&text));
    }

    let user_info: UserInfo = res.json().await.map_err(|e| JsValue::from_str(&e.to_string()))?;
    serde_wasm_bindgen::to_value(&user_info).map_err(|e| JsValue::from_str(&e.to_string()))
}

#[wasm_bindgen]
pub async fn new_post(title: String, content: String) -> Result<JsValue, JsValue> {
    if title.trim().is_empty() || content.trim().is_empty() {
        return check_error("You have to set title and content.");
    }

    let token = get_token_from_storage().ok_or_else(|| JsValue::from_str("You have to be authorized"))?;
    let body = serde_json::json!({"title": title, "content": content});
    let res = Request::post(&format!("{}{}{}{}{}", BASE_URL, PATH_BASE, PATH_PROTECTED, PATH_POSTS, PATH_POSTS_NEW))
        .header("Content-Type", "application/json")
        .header("Authorization", &format!("Bearer {token}"))
        .body(body.to_string())
        .map_err(|e| JsValue::from_str(&e.to_string()))?
        .send()
        .await
        .map_err(check_network_error)?;

    if !res.ok() {
        let text = get_error_msg(res, "Error while creating post").await;
        return Err(JsValue::from_str(&text));
    }

    let post_response: WasmPostResponse = res.json().await.map_err(|e| JsValue::from_str(&e.to_string()))?;
    serde_wasm_bindgen::to_value(&post_response.post).map_err(|e| JsValue::from_str(&e.to_string()))
}

#[wasm_bindgen]
pub async fn update_post(id: i64, title: String, content: String) -> Result<JsValue, JsValue> {
    if title.trim().is_empty() || content.trim().is_empty() {
        return check_error("You have to set title and content.");
    }

    let token = get_token_from_storage().ok_or_else(|| JsValue::from_str("You have to be authorized"))?;
    let body = serde_json::json!({"title": title, "content": content});
    let res = Request::put(&format!("{}{}{}{}/{}", BASE_URL, PATH_BASE, PATH_PROTECTED, PATH_POSTS, id))
        .header("Content-Type", "application/json")
        .header("Authorization", &format!("Bearer {token}"))
        .body(body.to_string())
        .map_err(|e| JsValue::from_str(&e.to_string()))?
        .send()
        .await
        .map_err(check_network_error)?;

    if !res.ok() {
        let text = get_error_msg(res, "Error while update post.").await;
        return Err(JsValue::from_str(&text));
    }

    let post_response: WasmPostResponse = res.json().await.map_err(|e| JsValue::from_str(&e.to_string()))?;
    serde_wasm_bindgen::to_value(&post_response.post).map_err(|e| JsValue::from_str(&e.to_string()))
}

#[wasm_bindgen]
pub async fn del_post(id: i64) -> Result<(), JsValue> {
    let token = get_token_from_storage().ok_or_else(|| JsValue::from_str("You have to be authorized"))?;
    let res = Request::delete(&format!("{}{}{}{}/{}", BASE_URL, PATH_BASE, PATH_PROTECTED, PATH_POSTS, id))
        .header("Authorization", &format!("Bearer {token}"))
        .send()
        .await
        .map_err(check_network_error)?;

    if !res.ok() {
        let text = get_error_msg(res, "Error while creating post").await;
        return Err(JsValue::from_str(&text));
    }
    Ok(())
}

#[wasm_bindgen]
pub async fn posts(limit: i64, offset: i64) -> Result<JsValue, JsValue> {
    let params = format!("?limit={}&offset={}", limit, offset);
    let res = Request::get(&format!("{}{}{}{}", BASE_URL, PATH_BASE, PATH_POSTS, params))
        .send()
        .await
        .map_err(check_network_error)?;
    if !res.ok() {
        let text = get_error_msg(res, "Eror while reading posts list").await;
        return Err(JsValue::from_str(&text));
    }

    let list: PostsList = res.json().await.map_err(|e| JsValue::from_str(&e.to_string()))?;
    serde_wasm_bindgen::to_value(&list).map_err(|e| JsValue::from_str(&e.to_string()))
}