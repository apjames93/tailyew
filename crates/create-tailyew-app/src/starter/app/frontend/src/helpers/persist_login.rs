use web_sys::window;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LoginResponse {
    pub token: String,
}

// Set a token to window session for AUTH
pub fn persist_login(user: &LoginResponse) {
    if let Some(window) = window() {
        if let Ok(Some(storage)) = window.session_storage() {
            storage.set_item("token", &user.token).unwrap();
        }
    }
}
