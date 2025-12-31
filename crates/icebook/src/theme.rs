//! Theme abstraction for icebook
//!
//! Consumers implement `ThemeProvider` to supply their own theme system.

use iced::Color;

/// Brightness mode for theme switching
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Brightness {
    #[default]
    Dark,
    Light,
}

impl Brightness {
    /// Toggle between Dark and Light
    pub fn toggle(&self) -> Self {
        match self {
            Brightness::Dark => Brightness::Light,
            Brightness::Light => Brightness::Dark,
        }
    }
}

/// Minimal theme interface for icebook's sidebar UI
///
/// Consumers can implement this for their theme type, or use the provided
/// `DefaultSidebarTheme` which works with any theme.
pub trait SidebarTheme {
    /// Background color for sidebar
    fn sidebar_background(&self) -> Color;
    /// Primary text color
    fn text_primary(&self) -> Color;
    /// Secondary/muted text color
    fn text_secondary(&self) -> Color;
    /// Background color for selected items
    fn selected_background(&self) -> Color;
    /// Background color for hovered items
    fn hover_background(&self) -> Color;
    /// Main content area background
    fn content_background(&self) -> Color;
}

/// Theme provider trait - implement this in your storybook consumer
///
/// This is the main integration point for custom themes. The `Theme` type
/// is passed to your story `view()` functions.
///
/// # Example
///
/// ```rust,ignore
/// use icebook::{Brightness, ThemeProvider, SidebarTheme};
/// use playtron_themes::ThemeInterface;
///
/// pub struct MyThemeProvider;
///
/// impl ThemeProvider for MyThemeProvider {
///     type Theme = dyn ThemeInterface;
///
///     fn get_theme(brightness: Brightness) -> &'static Self::Theme {
///         match brightness {
///             Brightness::Dark => &DARK_THEME,
///             Brightness::Light => &LIGHT_THEME,
///         }
///     }
///
///     fn get_sidebar_theme(brightness: Brightness) -> &'static dyn SidebarTheme {
///         Self::get_theme(brightness)
///     }
/// }
/// ```
pub trait ThemeProvider {
    /// The theme type passed to story view functions
    type Theme: ?Sized + 'static;

    /// Get the theme for the given brightness mode
    fn get_theme(brightness: Brightness) -> &'static Self::Theme;

    /// Get the sidebar theme for UI chrome
    /// Default implementation uses `SimpleSidebarTheme`
    fn get_sidebar_theme(brightness: Brightness) -> &'static dyn SidebarTheme {
        match brightness {
            Brightness::Dark => &DARK_SIDEBAR,
            Brightness::Light => &LIGHT_SIDEBAR,
        }
    }
}

/// Simple built-in sidebar theme for dark mode
pub struct SimpleDarkSidebar;

impl SidebarTheme for SimpleDarkSidebar {
    fn sidebar_background(&self) -> Color {
        Color::from_rgb(0.1, 0.1, 0.1)
    }
    fn text_primary(&self) -> Color {
        Color::from_rgb(0.95, 0.95, 0.95)
    }
    fn text_secondary(&self) -> Color {
        Color::from_rgb(0.6, 0.6, 0.6)
    }
    fn selected_background(&self) -> Color {
        Color::from_rgba(1.0, 1.0, 1.0, 0.1)
    }
    fn hover_background(&self) -> Color {
        Color::from_rgba(1.0, 1.0, 1.0, 0.05)
    }
    fn content_background(&self) -> Color {
        Color::from_rgb(0.15, 0.15, 0.15)
    }
}

/// Simple built-in sidebar theme for light mode
pub struct SimpleLightSidebar;

impl SidebarTheme for SimpleLightSidebar {
    fn sidebar_background(&self) -> Color {
        Color::from_rgb(0.95, 0.95, 0.95)
    }
    fn text_primary(&self) -> Color {
        Color::from_rgb(0.1, 0.1, 0.1)
    }
    fn text_secondary(&self) -> Color {
        Color::from_rgb(0.4, 0.4, 0.4)
    }
    fn selected_background(&self) -> Color {
        Color::from_rgba(0.0, 0.0, 0.0, 0.08)
    }
    fn hover_background(&self) -> Color {
        Color::from_rgba(0.0, 0.0, 0.0, 0.04)
    }
    fn content_background(&self) -> Color {
        Color::WHITE
    }
}

static DARK_SIDEBAR: SimpleDarkSidebar = SimpleDarkSidebar;
static LIGHT_SIDEBAR: SimpleLightSidebar = SimpleLightSidebar;

/// Returns the default sidebar theme for the given brightness.
/// Convenience function for ThemeProvider implementations that don't need custom sidebar styling.
pub fn default_sidebar_theme(brightness: Brightness) -> &'static dyn SidebarTheme {
    match brightness {
        Brightness::Dark => &DARK_SIDEBAR,
        Brightness::Light => &LIGHT_SIDEBAR,
    }
}
