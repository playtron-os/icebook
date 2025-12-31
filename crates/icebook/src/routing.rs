//! URL hash-based routing for persisting selected story across page refreshes

/// Get the initial route from URL hash (WASM only)
#[cfg(target_arch = "wasm32")]
pub fn get_initial_route() -> Option<String> {
    let window = web_sys::window()?;
    let location = window.location();
    let hash = location.hash().ok()?;

    // Strip leading # and / if present
    let route = hash.trim_start_matches('#').trim_start_matches('/');

    if route.is_empty() {
        None
    } else {
        Some(route.to_lowercase())
    }
}

/// Get initial route (native: always None)
#[cfg(not(target_arch = "wasm32"))]
pub fn get_initial_route() -> Option<String> {
    None
}

/// Update the URL hash when story changes (WASM only)
#[cfg(target_arch = "wasm32")]
pub fn set_url_hash(story_id: &str) {
    let window = match web_sys::window() {
        Some(w) => w,
        None => return,
    };

    let hash = if story_id.is_empty() || story_id == "welcome" {
        String::new()
    } else {
        format!("#/{}", story_id)
    };

    // Use history.replaceState to avoid adding to browser history on every click
    if let Ok(history) = window.history() {
        let _ = history.replace_state_with_url(&wasm_bindgen::JsValue::NULL, "", Some(&hash));
    }
}

/// Update URL hash (native: no-op)
#[cfg(not(target_arch = "wasm32"))]
pub fn set_url_hash(_story_id: &str) {
    // No URL routing on native
}
