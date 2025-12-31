//! Generic sidebar navigation component

use iced::widget::{button, column, container, text, Column, Space};
use iced::{Color, Element, Length};

use crate::theme::SidebarTheme;

/// A navigation item in the sidebar
#[derive(Debug, Clone)]
pub struct NavItem {
    pub id: String,
    pub label: String,
}

/// A section in the sidebar containing navigation items
#[derive(Debug, Clone)]
pub struct SidebarSection {
    pub title: String,
    pub items: Vec<NavItem>,
}

/// Configuration for the sidebar
#[derive(Debug, Clone)]
pub struct SidebarConfig {
    pub title: String,
    pub sections: Vec<SidebarSection>,
}

/// Messages from sidebar interactions
#[derive(Debug, Clone)]
pub enum SidebarMessage {
    ToggleBrightness,
    SelectStory(String),
}

/// Render the sidebar with component navigation
pub fn sidebar<'a>(
    config: &'a SidebarConfig,
    selected: &str,
    theme: &'a dyn SidebarTheme,
) -> Element<'a, SidebarMessage> {
    let bg_color = theme.sidebar_background();
    let text_color = theme.text_primary();
    let text_secondary = theme.text_secondary();

    // Get font configuration from theme
    let title_font = theme.title_font();
    let button_font = theme.button_font();

    let header = text(&config.title)
        .size(theme.title_size())
        .color(text_color)
        .font(title_font.font)
        .shaping(title_font.shaping);

    let theme_toggle = button(
        text("Toggle Theme")
            .color(text_secondary)
            .size(theme.button_size())
            .font(button_font.font)
            .shaping(button_font.shaping),
    )
    .on_press(SidebarMessage::ToggleBrightness)
    .padding(8);

    // Build component list from sections
    let mut components: Column<'a, SidebarMessage> = Column::new().spacing(4);

    for (i, section) in config.sections.iter().enumerate() {
        // Add spacing between sections (not before the first one)
        if i > 0 {
            components = components.push(Space::new().height(16));
        }

        // Section header
        components = components.push(section_header(&section.title, text_secondary, theme));

        // Navigation items in this section
        for item in &section.items {
            components = components.push(nav_item(&item.id, &item.label, selected, theme));
        }
    }

    let content = column![
        header,
        Space::new().height(8),
        theme_toggle,
        Space::new().height(24),
        components,
    ]
    .padding(16);

    container(content)
        .width(Length::Fixed(theme.sidebar_width()))
        .height(Length::Fill)
        .style(move |_| container::Style {
            background: Some(iced::Background::Color(bg_color)),
            ..Default::default()
        })
        .into()
}

fn section_header<'a>(
    label: &str,
    color: Color,
    theme: &'a dyn SidebarTheme,
) -> Element<'a, SidebarMessage> {
    let section_font = theme.section_font();
    text(label.to_string())
        .size(theme.section_size())
        .color(color)
        .font(section_font.font)
        .shaping(section_font.shaping)
        .into()
}

fn nav_item<'a>(
    id: &str,
    label: &str,
    selected: &str,
    theme: &'a dyn SidebarTheme,
) -> Element<'a, SidebarMessage> {
    let is_selected = id == selected;
    let text_color = if is_selected {
        theme.text_primary()
    } else {
        theme.text_secondary()
    };
    let bg_color = if is_selected {
        theme.selected_background()
    } else {
        Color::TRANSPARENT
    };
    let hover_bg = theme.hover_background();
    let nav_font = theme.nav_font();
    let nav_size = theme.nav_size();

    let id_owned = id.to_string();

    let btn = button(
        text(label.to_string())
            .size(nav_size)
            .color(text_color)
            .font(nav_font.font)
            .shaping(nav_font.shaping),
    )
    .on_press(SidebarMessage::SelectStory(id_owned))
    .padding([8, 12])
    .width(Length::Fill)
    .style(move |_, status| {
        let bg = match status {
            button::Status::Hovered if !is_selected => hover_bg,
            _ => bg_color,
        };
        button::Style {
            background: Some(iced::Background::Color(bg)),
            text_color,
            border: iced::Border::default().rounded(6),
            ..Default::default()
        }
    });

    btn.into()
}
