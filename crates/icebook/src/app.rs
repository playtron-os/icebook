//! Main Storybook application shell
//!
//! Generic over the StoryRegistry provided by the consumer.

use iced::widget::{column, container, row, scrollable, text};
use iced::{Element, Length, Size, Subscription, Task};

use crate::preferences::Preferences;
use crate::routing;
use crate::sidebar::{sidebar, NavItem, SidebarConfig, SidebarMessage, SidebarSection};
use crate::story::{StoryMeta, StoryRegistry};
use crate::theme::{Brightness, ThemeProvider};

/// The main Storybook application
pub struct Storybook<S>
where
    S: StoryRegistry,
{
    /// Component registry provided by consumer
    stories: S,
    /// Currently selected story
    selected: String,
    /// Current brightness mode
    brightness: Brightness,
    /// User preferences
    preferences: Preferences,
    /// Cached sidebar config (owned data)
    sidebar_config: SidebarConfig,
    /// Current search query for filtering components
    search_query: String,
}

/// Messages for the Storybook application
#[derive(Debug, Clone)]
pub enum Message<M> {
    /// Message from a story component
    Story(M),
    /// Toggle between light/dark mode
    ToggleBrightness,
    /// Select a story to display
    SelectStory(String),
    /// Search query changed
    SearchChanged(String),
}

impl<S> Storybook<S>
where
    S: StoryRegistry,
{
    /// Create a new Storybook with the given registry
    pub fn new() -> (Self, Task<Message<S::Message>>) {
        let stories = S::default();
        let preferences = Preferences::load();
        let brightness = preferences.brightness();

        // Build sidebar config from story metadata
        let story_list = S::stories();
        let sidebar_config = build_sidebar_config(S::title(), &story_list);

        // Check URL hash for initial story, otherwise use first story or empty for welcome
        let selected = routing::get_initial_route()
            .filter(|id| story_list.iter().any(|s| s.id == id))
            .or_else(|| story_list.first().map(|s| s.id.to_string()))
            .unwrap_or_default();

        // Sync URL to selected story (in case we defaulted to first)
        routing::set_url_hash(&selected);

        let app = Self {
            stories,
            selected,
            brightness,
            preferences,
            sidebar_config,
            search_query: String::new(),
        };

        (app, Task::none())
    }

    /// Update the application state
    pub fn update(&mut self, message: Message<S::Message>) -> Task<Message<S::Message>> {
        match message {
            Message::Story(msg) => {
                self.stories.update(&self.selected, msg);
                Task::none()
            }
            Message::ToggleBrightness => {
                self.brightness = self.brightness.toggle();
                self.preferences.set_brightness(self.brightness);
                self.preferences.save();
                Task::none()
            }
            Message::SelectStory(id) => {
                self.selected = id.clone();
                routing::set_url_hash(&id);
                Task::none()
            }
            Message::SearchChanged(query) => {
                self.search_query = query;
                Task::none()
            }
        }
    }

    /// Render the application view
    pub fn view(&self) -> Element<'_, Message<S::Message>> {
        // Get themes from the consumer's provider
        let theme = S::Provider::get_theme(self.brightness);
        let sidebar_theme = S::Provider::get_sidebar_theme(self.brightness);

        // Check if consumer provides a custom sidebar
        let sidebar_view = self
            .stories
            .sidebar_view(
                &self.sidebar_config,
                &self.selected,
                &self.search_query,
                sidebar_theme,
                theme,
            )
            .unwrap_or_else(|| {
                sidebar(
                    &self.sidebar_config,
                    &self.selected,
                    &self.search_query,
                    sidebar_theme,
                )
            })
            .map(|msg| match msg {
                SidebarMessage::ToggleBrightness => Message::ToggleBrightness,
                SidebarMessage::SelectStory(id) => Message::SelectStory(id),
                SidebarMessage::SearchChanged(query) => Message::SearchChanged(query),
            });

        // Render main content area
        let content = if self.selected.is_empty() {
            self.stories.welcome_view(theme).map(Message::Story)
        } else {
            self.stories.view(&self.selected, theme).map(Message::Story)
        };

        // Wrap content in scrollable area
        let content_scrollable = scrollable(
            container(content)
                .padding(32)
                .width(Length::Fill)
                .height(Length::Shrink),
        )
        .width(Length::Fill)
        .height(Length::Fill);

        // Layout: sidebar | content
        let layout = row![sidebar_view, content_scrollable];
        let bg_color = sidebar_theme.content_background();

        container(layout)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(move |_| container::Style {
                background: Some(iced::Background::Color(bg_color)),
                ..Default::default()
            })
            .into()
    }

    /// Get the Iced theme
    pub fn theme(&self) -> iced::Theme {
        match self.brightness {
            Brightness::Dark => iced::Theme::Dark,
            Brightness::Light => iced::Theme::Light,
        }
    }

    /// Application title
    #[allow(dead_code)]
    pub fn title(&self) -> String {
        S::title().to_string()
    }

    /// Window subscription
    pub fn subscription(&self) -> Subscription<Message<S::Message>> {
        Subscription::none()
    }
}

impl<S> Default for Storybook<S>
where
    S: StoryRegistry,
{
    fn default() -> Self {
        Self::new().0
    }
}

/// Build sidebar configuration from story metadata (owned Strings)
fn build_sidebar_config(title: &str, stories: &[StoryMeta]) -> SidebarConfig {
    // Group stories by category
    let mut categories: std::collections::BTreeMap<String, Vec<NavItem>> =
        std::collections::BTreeMap::new();

    for story in stories {
        let category = story.category.to_string();
        categories.entry(category).or_default().push(NavItem {
            id: story.id.to_string(),
            label: story.title.to_string(),
        });
    }

    let sections: Vec<SidebarSection> = categories
        .into_iter()
        .map(|(title, items)| SidebarSection { title, items })
        .collect();

    SidebarConfig {
        title: title.to_string(),
        sections,
    }
}

/// Default welcome view when no story is selected
///
/// This provides a simple welcome message that works with any message type.
/// For more customization, override `StoryRegistry::welcome_view()` in your implementation.
///
/// # Arguments
/// * `title` - The storybook title to display in the welcome message
pub fn default_welcome_view<'a, M: 'a>(title: &str) -> Element<'a, M> {
    let text_color = iced::Color::from_rgb(0.7, 0.7, 0.7);
    let text_secondary = iced::Color::from_rgb(0.5, 0.5, 0.5);

    let welcome_text = format!("Welcome to {}", title);

    container(
        column![
            text(welcome_text).size(32).color(text_color),
            text("Select a component from the sidebar to view its stories.")
                .size(16)
                .color(text_secondary),
        ]
        .spacing(16)
        .padding(32),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .center_x(Length::Fill)
    .center_y(Length::Fill)
    .into()
}

/// Settings for window/application
#[derive(Debug, Clone, Copy)]
pub struct Settings {
    /// Window title
    pub title: &'static str,
    /// Initial window size
    pub window_size: Size,
    /// Font bytes to load (use `include_bytes!` for WASM)
    /// Pass multiple fonts for different weights/styles
    pub fonts: &'static [&'static [u8]],
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            title: "icebook",
            window_size: Size::new(1200.0, 800.0),
            fonts: &[],
        }
    }
}
