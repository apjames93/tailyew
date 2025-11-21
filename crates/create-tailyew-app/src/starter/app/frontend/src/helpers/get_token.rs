use web_sys::window;

pub fn get_token() -> Option<String> {
    if let Some(window) = window() {
        if let Ok(Some(storage)) = window.session_storage() {
            return storage.get_item("token").unwrap_or(None);
        }
    }
    None
}
