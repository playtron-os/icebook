//! User preferences with localStorage persistence (WASM) and system theme detection

use crate::theme::Brightness;

#[cfg(target_arch = "wasm32")]
const THEME_KEY: &str = "icebook_theme";

/// User preferences for the storybook
#[derive(Debug, Clone)]
pub struct Preferences {
    brightness: Brightness,
}

impl Preferences {
    /// Load preferences (from localStorage on WASM, defaults on native)
    pub fn load() -> Self {
        Self {
            brightness: get_initial_brightness(),
        }
    }

    /// Get the current brightness preference
    pub fn brightness(&self) -> Brightness {
        self.brightness
    }

    /// Set the brightness preference
    pub fn set_brightness(&mut self, brightness: Brightness) {
        self.brightness = brightness;
    }

    /// Save preferences (to localStorage on WASM, no-op on native)
    pub fn save(&self) {
        save_brightness(self.brightness);
    }
}

impl Default for Preferences {
    fn default() -> Self {
        Self::load()
    }
}

/// Get the initial brightness based on: saved preference > system preference > default (dark)
pub fn get_initial_brightness() -> Brightness {
    // First check if user has a saved preference
    if let Some(saved) = load_brightness() {
        return saved;
    }

    // Fall back to system preference
    get_system_brightness()
}

/// Get the system/browser color scheme preference
#[cfg(target_arch = "wasm32")]
fn get_system_brightness() -> Brightness {
    let window = match web_sys::window() {
        Some(w) => w,
        None => return Brightness::Dark,
    };

    let result = window.match_media("(prefers-color-scheme: dark)");

    match result {
        Ok(Some(media_query)) => {
            if media_query.matches() {
                Brightness::Dark
            } else {
                Brightness::Light
            }
        }
        _ => Brightness::Dark,
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn get_system_brightness() -> Brightness {
    Brightness::Dark
}

/// Save brightness preference
#[cfg(target_arch = "wasm32")]
pub fn save_brightness(brightness: Brightness) {
    let window = match web_sys::window() {
        Some(w) => w,
        None => return,
    };

    let storage = match window.local_storage() {
        Ok(Some(s)) => s,
        _ => return,
    };

    let value = match brightness {
        Brightness::Dark => "dark",
        Brightness::Light => "light",
    };

    let _ = storage.set_item(THEME_KEY, value);
}

#[cfg(not(target_arch = "wasm32"))]
pub fn save_brightness(_brightness: Brightness) {
    // No persistence on native
}

/// Load brightness preference
#[cfg(target_arch = "wasm32")]
pub fn load_brightness() -> Option<Brightness> {
    let window = web_sys::window()?;
    let storage = window.local_storage().ok()??;
    let value = storage.get_item(THEME_KEY).ok()??;

    match value.as_str() {
        "dark" => Some(Brightness::Dark),
        "light" => Some(Brightness::Light),
        _ => None,
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn load_brightness() -> Option<Brightness> {
    None
}
