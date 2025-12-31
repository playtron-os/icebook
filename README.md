# Icebook

A **theme-agnostic** component storybook framework for [Iced](https://iced.rs) applications. Compiles to WASM for web deployment.

## Overview

Icebook provides a reusable storybook infrastructure that any Iced-based component library can use to document and showcase its components. Unlike other storybook frameworks, icebook is **generic over your theme system** - bring your own themes via the `ThemeProvider` trait.

### Features

- 📚 **Story Organization** - Group stories by category with automatic sidebar
- 🎨 **Theme Plugin System** - Bring your own theme type via `ThemeProvider` trait
- 🌓 **Light/Dark Mode** - System preference detection with toggle
- 💾 **Preference Persistence** - Remember theme choice in localStorage (WASM)
- 🌐 **WASM-First** - Optimized for web deployment via trunk

## Quick Start

### 1. Add dependencies

```toml
[dependencies]
icebook = { path = "path/to/icebook/crates/icebook" }
# Your theme library (e.g., playtron_themes)
my_themes = "..."
once_cell = "1.19"  # For static theme instances
```

### 2. Implement ThemeProvider

```rust
use icebook::{Brightness, SidebarTheme, ThemeProvider};
use my_themes::MyTheme;

pub struct MyThemeProvider;

impl ThemeProvider for MyThemeProvider {
    type Theme = dyn MyTheme;  // Your theme trait/type

    fn get_theme(brightness: Brightness) -> &'static Self::Theme {
        static DARK: Lazy<DarkTheme> = Lazy::new(DarkTheme::new);
        static LIGHT: Lazy<LightTheme> = Lazy::new(LightTheme::new);
        
        match brightness {
            Brightness::Dark => &*DARK,
            Brightness::Light => &*LIGHT,
        }
    }

    // Optional: customize sidebar colors
    fn get_sidebar_theme(brightness: Brightness) -> &'static dyn SidebarTheme {
        icebook::default_sidebar_theme(brightness)  // Use built-in
    }
}
```

### 3. Define Stories

```rust
use icebook::{Story, StoryMeta};
use my_themes::MyTheme;

#[derive(Default)]
pub struct ButtonsStory {
    click_count: u32,
}

#[derive(Debug, Clone)]
pub enum ButtonsMessage {
    Clicked,
}

impl Story<dyn MyTheme> for ButtonsStory {
    type Message = ButtonsMessage;

    fn meta() -> StoryMeta {
        StoryMeta {
            id: "buttons",
            title: "Buttons",
            category: "Actions",
        }
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            ButtonsMessage::Clicked => self.click_count += 1,
        }
    }

    fn view<'a>(&'a self, theme: &'a dyn MyTheme) -> Element<'a, Self::Message> {
        // Render your component demos with theme...
    }
}
```

### 4. Create StoryRegistry

```rust
use icebook::{StoryMeta, StoryRegistry, ThemeProvider};

#[derive(Default)]
pub struct MyStories {
    buttons: ButtonsStory,
    cards: CardsStory,
}

#[derive(Debug, Clone)]
pub enum MyStoriesMessage {
    Buttons(ButtonsMessage),
    Cards(CardsMessage),
}

impl StoryRegistry for MyStories {
    type Message = MyStoriesMessage;
    type Provider = MyThemeProvider;  // Link to your theme provider

    fn stories() -> Vec<StoryMeta> {
        vec![
            ButtonsStory::meta(),
            CardsStory::meta(),
        ]
    }

    fn update(&mut self, story_id: &str, message: Self::Message) {
        match (story_id, message) {
            ("buttons", MyStoriesMessage::Buttons(msg)) => self.buttons.update(msg),
            ("cards", MyStoriesMessage::Cards(msg)) => self.cards.update(msg),
            _ => {}
        }
    }

    fn view<'a>(
        &'a self,
        story_id: &str,
        theme: &'a <Self::Provider as ThemeProvider>::Theme,
    ) -> Element<'a, Self::Message> {
        match story_id {
            "buttons" => self.buttons.view(theme).map(MyStoriesMessage::Buttons),
            "cards" => self.cards.view(theme).map(MyStoriesMessage::Cards),
            _ => text("Story not found").into(),
        }
    }

    fn welcome_view<'a>(
        &self,
        theme: &'a <Self::Provider as ThemeProvider>::Theme,
    ) -> Element<'a, Self::Message> {
        // Your welcome page...
    }
}
```

### 5. Run the storybook

```rust
// Native
fn main() -> iced::Result {
    icebook::run::<MyStories>()
}

// WASM
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn main() {
    let _ = icebook::run::<MyStories>();
}
```

## Architecture

### Theme Plugin System

The key abstraction is `ThemeProvider`:

```rust
pub trait ThemeProvider {
    /// Your theme type - passed to story view() methods
    type Theme: ?Sized + 'static;

    /// Get theme for brightness mode
    fn get_theme(brightness: Brightness) -> &'static Self::Theme;

    /// Get sidebar theme (has sensible default)
    fn get_sidebar_theme(brightness: Brightness) -> &'static dyn SidebarTheme;
}
```

This allows icebook to:
1. Be completely independent of any specific theme library
2. Pass your actual theme type to stories (not a wrapper)
3. Provide default sidebar styling while allowing customization

### Workspace Structure

```
icebook/
├── Cargo.toml              # Workspace root
└── crates/
    └── icebook/            # Main library
        └── src/
            ├── lib.rs      # Public API, run()
            ├── app.rs      # Storybook<S> generic app
            ├── story.rs    # Story, StoryMeta, StoryRegistry
            ├── theme.rs    # Brightness, SidebarTheme, ThemeProvider
            ├── sidebar.rs  # Navigation sidebar
            └── preferences.rs  # Theme persistence
```

## Building for WASM

Use [trunk](https://trunkrs.dev/) for WASM builds:

```bash
# Install trunk
cargo install trunk

# Serve locally with hot-reload
trunk serve

# Build for production
trunk build --release
```

## Example: icetron-storybook

See `icetron/crates/icetron-storybook` for a complete example using `playtron_themes`.
