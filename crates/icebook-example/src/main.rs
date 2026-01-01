//! Example storybook demonstrating icebook usage

use icebook::prelude::*;
use iced::widget::{button, column, container, row, text, text_input};
use iced::{Color, Element, Length};

// ============================================================================
// Theme Provider
// ============================================================================

/// Simple theme for the example
pub struct SimpleTheme {
    pub background: Color,
    pub text: Color,
    pub primary: Color,
}

impl SimpleTheme {
    pub fn dark() -> Self {
        Self {
            background: Color::from_rgb(0.1, 0.1, 0.1),
            text: Color::WHITE,
            primary: Color::from_rgb(0.3, 0.5, 1.0),
        }
    }

    pub fn light() -> Self {
        Self {
            background: Color::WHITE,
            text: Color::BLACK,
            primary: Color::from_rgb(0.2, 0.4, 0.9),
        }
    }
}

/// Theme provider implementation
pub struct SimpleThemeProvider;

static DARK_THEME: once_cell::sync::Lazy<SimpleTheme> =
    once_cell::sync::Lazy::new(SimpleTheme::dark);
static LIGHT_THEME: once_cell::sync::Lazy<SimpleTheme> =
    once_cell::sync::Lazy::new(SimpleTheme::light);

impl ThemeProvider for SimpleThemeProvider {
    type Theme = SimpleTheme;

    fn get_theme(brightness: Brightness) -> &'static Self::Theme {
        match brightness {
            Brightness::Dark => &*DARK_THEME,
            Brightness::Light => &*LIGHT_THEME,
        }
    }
}

// ============================================================================
// Stories
// ============================================================================

/// Button story
#[derive(Default)]
pub struct ButtonStory {
    click_count: usize,
}

#[derive(Debug, Clone)]
pub enum ButtonMessage {
    Clicked,
}

impl Story<SimpleTheme> for ButtonStory {
    type Message = ButtonMessage;

    fn meta() -> StoryMeta {
        StoryMeta {
            id: "buttons",
            title: "Buttons",
            category: "Components",
        }
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            ButtonMessage::Clicked => {
                self.click_count += 1;
            }
        }
    }

    fn view(&self, theme: &SimpleTheme) -> Element<'_, Self::Message> {
        let primary = theme.primary;
        column![
            text("Button Story").size(24).color(theme.text),
            text(format!("Click count: {}", self.click_count)).color(theme.text),
            button(text("Click me!").color(theme.text))
                .on_press(ButtonMessage::Clicked)
                .padding(10),
            button(text("Primary Button").color(Color::WHITE))
                .on_press(ButtonMessage::Clicked)
                .padding(10)
                .style(move |_theme, _status| button::Style {
                    background: Some(primary.into()),
                    text_color: Color::WHITE,
                    ..Default::default()
                }),
        ]
        .spacing(16)
        .into()
    }
}

/// Input story
#[derive(Default)]
pub struct InputStory {
    text_value: String,
}

#[derive(Debug, Clone)]
pub enum InputMessage {
    TextChanged(String),
}

impl Story<SimpleTheme> for InputStory {
    type Message = InputMessage;

    fn meta() -> StoryMeta {
        StoryMeta {
            id: "inputs",
            title: "Inputs",
            category: "Components",
        }
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            InputMessage::TextChanged(value) => {
                self.text_value = value;
            }
        }
    }

    fn view(&self, theme: &SimpleTheme) -> Element<'_, Self::Message> {
        column![
            text("Input Story").size(24).color(theme.text),
            text("Enter some text:").color(theme.text),
            text_input("Placeholder...", &self.text_value)
                .on_input(InputMessage::TextChanged)
                .padding(10)
                .width(Length::Fixed(300.0)),
            text(format!("You typed: {}", self.text_value)).color(theme.text),
        ]
        .spacing(16)
        .into()
    }
}

/// Typography story
#[derive(Default)]
pub struct TypographyStory;

#[derive(Debug, Clone)]
pub enum TypographyMessage {}

impl Story<SimpleTheme> for TypographyStory {
    type Message = TypographyMessage;

    fn meta() -> StoryMeta {
        StoryMeta {
            id: "typography",
            title: "Typography",
            category: "Foundation",
        }
    }

    fn update(&mut self, _message: Self::Message) {}

    fn view(&self, theme: &SimpleTheme) -> Element<'_, Self::Message> {
        column![
            text("Typography Story").size(32).color(theme.text),
            text("Heading 1").size(28).color(theme.text),
            text("Heading 2").size(24).color(theme.text),
            text("Heading 3").size(20).color(theme.text),
            text("Body text - The quick brown fox jumps over the lazy dog.")
                .size(16)
                .color(theme.text),
            text("Caption text").size(12).color(theme.text),
        ]
        .spacing(12)
        .into()
    }
}

/// Colors story
#[derive(Default)]
pub struct ColorsStory;

#[derive(Debug, Clone)]
pub enum ColorsMessage {}

impl Story<SimpleTheme> for ColorsStory {
    type Message = ColorsMessage;

    fn meta() -> StoryMeta {
        StoryMeta {
            id: "colors",
            title: "Colors",
            category: "Foundation",
        }
    }

    fn update(&mut self, _message: Self::Message) {}

    fn view(&self, theme: &SimpleTheme) -> Element<'_, Self::Message> {
        let text_color = theme.text;

        column![
            text("Colors Story").size(24).color(text_color),
            Self::color_swatch(theme.background, "Background", text_color),
            Self::color_swatch(theme.text, "Text", text_color),
            Self::color_swatch(theme.primary, "Primary", text_color),
            Self::color_swatch(Color::from_rgb(0.2, 0.8, 0.2), "Success", text_color),
            Self::color_swatch(Color::from_rgb(0.9, 0.2, 0.2), "Error", text_color),
            Self::color_swatch(Color::from_rgb(0.9, 0.7, 0.1), "Warning", text_color),
        ]
        .spacing(12)
        .into()
    }
}

impl ColorsStory {
    fn color_swatch<'a>(
        color: Color,
        name: &'a str,
        text_color: Color,
    ) -> Element<'a, ColorsMessage> {
        row![
            container(text(""))
                .width(50)
                .height(50)
                .style(move |_| container::Style {
                    background: Some(color.into()),
                    ..Default::default()
                }),
            text(name).color(text_color),
        ]
        .spacing(8)
        .align_y(iced::Alignment::Center)
        .into()
    }
}

// ============================================================================
// Story Registry
// ============================================================================

#[derive(Debug, Clone)]
pub enum ExampleMessage {
    Button(ButtonMessage),
    Input(InputMessage),
    Typography(TypographyMessage),
    Colors(ColorsMessage),
}

#[derive(Default)]
pub struct ExampleStories {
    buttons: ButtonStory,
    inputs: InputStory,
    typography: TypographyStory,
    colors: ColorsStory,
}

impl StoryRegistry for ExampleStories {
    type Message = ExampleMessage;
    type Provider = SimpleThemeProvider;

    fn title() -> &'static str {
        "Example Storybook"
    }

    fn stories() -> Vec<StoryMeta> {
        vec![
            ButtonStory::meta(),
            InputStory::meta(),
            TypographyStory::meta(),
            ColorsStory::meta(),
        ]
    }

    fn update(&mut self, story_id: &str, message: Self::Message) {
        match (story_id, message) {
            ("buttons", ExampleMessage::Button(msg)) => self.buttons.update(msg),
            ("inputs", ExampleMessage::Input(msg)) => self.inputs.update(msg),
            ("typography", ExampleMessage::Typography(msg)) => self.typography.update(msg),
            ("colors", ExampleMessage::Colors(msg)) => self.colors.update(msg),
            _ => {}
        }
    }

    fn view<'a>(&'a self, story_id: &str, theme: &'a SimpleTheme) -> Element<'a, Self::Message> {
        match story_id {
            "buttons" => self.buttons.view(theme).map(ExampleMessage::Button),
            "inputs" => self.inputs.view(theme).map(ExampleMessage::Input),
            "typography" => self.typography.view(theme).map(ExampleMessage::Typography),
            "colors" => self.colors.view(theme).map(ExampleMessage::Colors),
            _ => text("Story not found").into(),
        }
    }
}

// ============================================================================
// Main
// ============================================================================

fn main() -> iced::Result {
    icebook::run::<ExampleStories>()
}
