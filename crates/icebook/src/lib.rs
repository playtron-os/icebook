//! icebook - A theme-agnostic storybook for Iced components
//!
//! # Overview
//!
//! icebook provides a framework for building component storybooks with Iced.
//! It's designed to be generic over your theme system - bring your own themes
//! by implementing the `ThemeProvider` trait.
//!
//! # Quick Start
//!
//! ```rust,ignore
//! use icebook::prelude::*;
//!
//! // Define your theme provider
//! struct MyThemeProvider;
//!
//! impl ThemeProvider for MyThemeProvider {
//!     type Theme = MyTheme;
//!
//!     fn get_theme(brightness: Brightness) -> &'static Self::Theme {
//!         // Return your theme based on brightness
//!     }
//!
//!     fn get_sidebar_theme(brightness: Brightness) -> &'static dyn SidebarTheme {
//!         // Return sidebar theming (can use defaults)
//!         icebook::default_sidebar_theme(brightness)
//!     }
//! }
//!
//! // Define your story registry
//! #[derive(Default)]
//! struct MyStories { /* ... */ }
//!
//! impl StoryRegistry for MyStories {
//!     type Message = MyMessage;
//!     type Provider = MyThemeProvider;
//!
//!     fn stories() -> Vec<StoryMeta> { /* ... */ }
//!     fn view(&self, story_id: &str, theme: &Self::Theme) -> Element<Self::Message> { /* ... */ }
//!     // ...
//! }
//!
//! // Run the storybook
//! fn main() -> iced::Result {
//!     icebook::run::<MyStories>()
//! }
//! ```
//!
//! # Architecture
//!
//! - **StoryRegistry**: Trait that your storybook must implement. Provides story metadata
//!   and rendering functions.
//! - **ThemeProvider**: Trait that supplies themes. Your registry specifies which provider to use.
//! - **SidebarTheme**: Minimal theme trait for the sidebar UI. Default implementations provided.
//! - **Storybook**: The main application shell that displays stories.
//!

mod app;
mod preferences;
mod sidebar;
mod story;
mod theme;

pub use app::{default_welcome_view, Message, Settings, Storybook};
pub use sidebar::{NavItem, SidebarConfig, SidebarMessage, SidebarSection};
pub use story::{Story, StoryMeta, StoryRegistry};
pub use theme::{
    default_sidebar_theme, Brightness, SidebarFont, SidebarTheme, SimpleDarkSidebar,
    SimpleLightSidebar, ThemeProvider,
};

/// Built-in fallback font (Fira Sans Regular)
///
/// This font is loaded automatically to ensure text renders in WASM environments
/// where system fonts may not be available. Iced's `Font::DEFAULT` uses
/// `Family::SansSerif`, so having a sans-serif font available is essential.
///
/// You can use this constant in your theme's font configuration:
/// ```rust,ignore
/// Font::with_name("Fira Sans")
/// ```
pub const FALLBACK_FONT: &[u8] = include_bytes!("../assets/fonts/FiraSans-Regular.ttf");

/// Name of the fallback font family
pub const FALLBACK_FONT_NAME: &str = "Fira Sans";

/// Prelude for convenient imports
pub mod prelude {
    pub use crate::app::{default_welcome_view, Message, Settings, Storybook};
    pub use crate::run;
    pub use crate::sidebar::{NavItem, SidebarConfig, SidebarMessage, SidebarSection};
    pub use crate::story::{Story, StoryMeta, StoryRegistry};
    pub use crate::theme::{
        default_sidebar_theme, Brightness, SidebarFont, SidebarTheme, SimpleDarkSidebar,
        SimpleLightSidebar, ThemeProvider,
    };
    pub use crate::{FALLBACK_FONT, FALLBACK_FONT_NAME};
}

/// Initialize WASM environment (panic hook, tracing)
#[cfg(target_arch = "wasm32")]
fn init_wasm() {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();
}

/// Run the storybook application
///
/// This is the main entry point for running your storybook.
/// Provide your `StoryRegistry` implementation as the type parameter.
///
/// # Example
///
/// ```rust,ignore
/// fn main() -> iced::Result {
///     icebook::run::<MyStories>()
/// }
/// ```
pub fn run<S>() -> iced::Result
where
    S: StoryRegistry + 'static,
    S::Message: std::fmt::Debug + Clone + Send,
{
    run_with_settings::<S>(Settings::default())
}

/// Run the storybook with custom settings
///
/// # Example
///
/// ```rust,ignore
/// fn main() -> iced::Result {
///     icebook::run_with_settings::<MyStories>(icebook::Settings {
///         title: "My Component Library",
///         window_size: iced::Size::new(1400.0, 900.0),
///         ..Default::default()
///     })
/// }
/// ```
pub fn run_with_settings<S>(settings: Settings) -> iced::Result
where
    S: StoryRegistry + 'static,
    S::Message: std::fmt::Debug + Clone + Send,
{
    #[cfg(target_arch = "wasm32")]
    init_wasm();

    let mut app = iced::application(
        Storybook::<S>::default,
        Storybook::<S>::update,
        Storybook::<S>::view,
    )
    .title(settings.title)
    .theme(Storybook::<S>::theme)
    .subscription(Storybook::<S>::subscription)
    .window_size(settings.window_size)
    // Always load the fallback font first for WASM compatibility
    .font(FALLBACK_FONT);

    // Load all custom fonts from consumer
    for font_bytes in settings.fonts {
        app = app.font(*font_bytes);
    }

    app.run()
}
