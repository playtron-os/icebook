//! Story trait and registry for defining component documentation

use iced::Element;

use crate::theme::ThemeProvider;

/// Metadata for a story, used for sidebar navigation and routing
#[derive(Debug, Clone)]
pub struct StoryMeta {
    /// Unique identifier/route for this story (e.g., "buttons")
    pub id: &'static str,
    /// Display title (e.g., "Buttons")
    pub title: &'static str,
    /// Category for grouping in sidebar (e.g., "Actions")
    pub category: &'static str,
}

/// Individual story trait for single component documentation
///
/// This is a convenience trait for organizing individual stories.
/// The `Theme` type parameter allows stories to work with any theme system.
///
/// # Example
///
/// ```rust,ignore
/// use icebook::{Story, StoryMeta};
///
/// #[derive(Default)]
/// pub struct ButtonsStory { click_count: u32 }
///
/// impl<T: ?Sized> Story<T> for ButtonsStory {
///     type Message = ButtonsMessage;
///
///     fn meta() -> StoryMeta {
///         StoryMeta {
///             id: "buttons",
///             title: "Buttons",
///             category: "Actions",
///         }
///     }
///
///     fn update(&mut self, message: Self::Message) {
///         // Handle messages
///     }
///
///     fn view<'a>(&'a self, theme: &'a T) -> Element<'a, Self::Message> {
///         // Render story
///     }
/// }
/// ```
pub trait Story<Theme: ?Sized> {
    /// Message type for this story
    type Message: Clone;

    /// Get metadata for this story
    fn meta() -> StoryMeta;

    /// Update state based on message
    fn update(&mut self, message: Self::Message);

    /// Render the story view
    fn view<'a>(&'a self, theme: &'a Theme) -> Element<'a, Self::Message>;
}

/// Registry of all stories in a storybook
///
/// This trait is implemented by the container that holds all story states.
/// The `Theme` type is provided by your `ThemeProvider` implementation.
///
/// # Example
///
/// ```rust,ignore
/// use icebook::{StoryMeta, StoryRegistry, ThemeProvider, Brightness};
///
/// #[derive(Default)]
/// pub struct MyStories {
///     buttons: ButtonsStory,
///     cards: CardsStory,
/// }
///
/// impl StoryRegistry for MyStories {
///     type Message = MyStoriesMessage;
///     type Provider = MyThemeProvider;
///
///     fn stories() -> Vec<StoryMeta> {
///         vec![/* ... */]
///     }
///
///     fn update(&mut self, story_id: &str, message: Self::Message) {
///         // Route messages to stories
///     }
///
///     fn view<'a>(
///         &'a self,
///         story_id: &str,
///         theme: &'a <Self::Provider as ThemeProvider>::Theme,
///     ) -> Element<'a, Self::Message> {
///         // Render the selected story
///     }
///
///     fn welcome_view<'a>(
///         &self,
///         theme: &'a <Self::Provider as ThemeProvider>::Theme,
///     ) -> Element<'a, Self::Message> {
///         // Render welcome page
///     }
/// }
/// ```
pub trait StoryRegistry: Default {
    /// Combined message type for all stories
    type Message: std::fmt::Debug + Clone + Send + 'static;

    /// Theme provider that supplies the theme type
    type Provider: ThemeProvider;

    /// Get metadata for all registered stories
    fn stories() -> Vec<StoryMeta>;

    /// Get the storybook title (shown in sidebar header and window title)
    /// Override this to customize the title (defaults to "icebook")
    fn title() -> &'static str {
        "icebook"
    }

    /// Get the welcome/home story ID (defaults to "welcome")
    fn welcome_id() -> &'static str {
        "welcome"
    }

    /// Update a specific story's state
    fn update(&mut self, story_id: &str, message: Self::Message);

    /// Render a specific story's view
    fn view<'a>(
        &'a self,
        story_id: &str,
        theme: &'a <Self::Provider as ThemeProvider>::Theme,
    ) -> Element<'a, Self::Message>;

    /// Render the welcome/home view
    /// Override this in your implementation to customize the welcome page.
    /// The default implementation displays a simple "Welcome to {title}" message.
    fn welcome_view<'a>(
        &self,
        _theme: &'a <Self::Provider as ThemeProvider>::Theme,
    ) -> Element<'a, Self::Message> {
        crate::app::default_welcome_view(Self::title())
    }
}
