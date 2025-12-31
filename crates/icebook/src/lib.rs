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
pub use story::{Story, StoryMeta, StoryRegistry};
pub use theme::{
    default_sidebar_theme, Brightness, SidebarTheme, SimpleDarkSidebar, SimpleLightSidebar,
    ThemeProvider,
};

/// Prelude for convenient imports
pub mod prelude {
    pub use crate::app::{default_welcome_view, Message, Settings, Storybook};
    pub use crate::run;
    pub use crate::story::{Story, StoryMeta, StoryRegistry};
    pub use crate::theme::{
        default_sidebar_theme, Brightness, SidebarTheme, SimpleDarkSidebar, SimpleLightSidebar,
        ThemeProvider,
    };
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
///         title: "My Component Library".to_string(),
///         window_size: iced::Size::new(1400.0, 900.0),
///     })
/// }
/// ```
pub fn run_with_settings<S>(_settings: Settings) -> iced::Result
where
    S: StoryRegistry + 'static,
    S::Message: std::fmt::Debug + Clone + Send,
{
    iced::application(
        Storybook::<S>::default,
        Storybook::<S>::update,
        Storybook::<S>::view,
    )
    .title("icebook")
    .theme(Storybook::<S>::theme)
    .subscription(Storybook::<S>::subscription)
    .run()
}
