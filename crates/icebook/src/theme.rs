//! Theme abstraction for icebook
//!
//! Consumers implement `ThemeProvider` to supply their own theme system.

use iced::widget::text::Shaping;
use iced::{Color, Font};

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

/// Font configuration for sidebar text
#[derive(Debug, Clone, Copy)]
pub struct SidebarFont {
    pub font: Font,
    pub shaping: Shaping,
}

impl Default for SidebarFont {
    fn default() -> Self {
        Self {
            font: Font::DEFAULT,
            shaping: Shaping::Basic,
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

    // === Typography configuration ===

    /// Font for the sidebar title
    fn title_font(&self) -> SidebarFont {
        SidebarFont::default()
    }
    /// Font size for the sidebar title
    fn title_size(&self) -> f32 {
        24.0
    }

    /// Font for section headers
    fn section_font(&self) -> SidebarFont {
        SidebarFont::default()
    }
    /// Font size for section headers
    fn section_size(&self) -> f32 {
        12.0
    }

    /// Font for navigation items
    fn nav_font(&self) -> SidebarFont {
        SidebarFont::default()
    }
    /// Font size for navigation items
    fn nav_size(&self) -> f32 {
        14.0
    }

    /// Font for the theme toggle button
    fn button_font(&self) -> SidebarFont {
        SidebarFont::default()
    }
    /// Font size for the theme toggle button
    fn button_size(&self) -> f32 {
        14.0
    }

    /// Sidebar width
    fn sidebar_width(&self) -> f32 {
        220.0
    }
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

/// Font configuration using the built-in Fira Sans fallback font
fn fallback_sidebar_font() -> SidebarFont {
    SidebarFont {
        font: Font::with_name(crate::FALLBACK_FONT_NAME),
        shaping: Shaping::Advanced,
    }
}

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

    // Use the built-in Fira Sans font for all text
    fn title_font(&self) -> SidebarFont {
        fallback_sidebar_font()
    }
    fn section_font(&self) -> SidebarFont {
        fallback_sidebar_font()
    }
    fn nav_font(&self) -> SidebarFont {
        fallback_sidebar_font()
    }
    fn button_font(&self) -> SidebarFont {
        fallback_sidebar_font()
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

    // Use the built-in Fira Sans font for all text
    fn title_font(&self) -> SidebarFont {
        fallback_sidebar_font()
    }
    fn section_font(&self) -> SidebarFont {
        fallback_sidebar_font()
    }
    fn nav_font(&self) -> SidebarFont {
        fallback_sidebar_font()
    }
    fn button_font(&self) -> SidebarFont {
        fallback_sidebar_font()
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
