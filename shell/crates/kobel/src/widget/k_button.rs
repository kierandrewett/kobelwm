use std::{path::PathBuf, sync::Arc};

use iced::widget::{container, row, svg};
use iced::{Background, Color, Element, Padding, Theme};

use crate::util::debug::debug_border_style_or_default;
use crate::widget::primitives::button;
use crate::{state::KobelShellState, KobelRootMessage};

#[derive(Default)]
pub enum KobelShellButtonMode {
    Iconic,
    Text,
    #[default]
    IconicAndText,
    MenuItem,
}

#[derive(Default)]
pub enum KobelShellButtonType {
    #[default]
    Normal,
    Primary,
}

pub struct KobelShellButton<'a> {
    state: &'a Arc<KobelShellState>,
    content: Element<'a, KobelRootMessage, iced::Theme, iced::Renderer>,
    mode: KobelShellButtonMode,
    button_type: KobelShellButtonType,
    radii: f32,
    on_press: KobelRootMessage
}

impl<'a> From<KobelShellButton<'a>> for Element<'a, KobelRootMessage, iced::Theme, iced::Renderer> {
    fn from(button: KobelShellButton<'a>) -> Self {
        button.view()
    }
}

impl<'a> KobelShellButton<'a> {
    pub fn new(state: &'a Arc<KobelShellState>, content: impl Into<Element<'a, KobelRootMessage, iced::Theme, iced::Renderer>>) -> Self {
        Self {
            state,
            content: content.into(),
            mode: KobelShellButtonMode::default(),
            button_type: KobelShellButtonType::default(),
            radii: 8.0,
            on_press: KobelRootMessage::Noop,
        }
    }

    pub fn mode(mut self, mode: KobelShellButtonMode) -> Self {
        self.mode = mode;
        self
    }

    pub fn radii(mut self, radii: f32) -> Self {
        self.radii = radii;
        self
    }

    pub fn on_press(mut self, on_press: KobelRootMessage) -> Self {
        self.on_press = on_press;
        self
    }

    pub fn button_type(mut self, button_type: KobelShellButtonType) -> Self {
        self.button_type = button_type;
        self
    }

    pub fn view(self) -> Element<'a, KobelRootMessage, iced::Theme, iced::Renderer> {
        let button_padding = match self.mode {
            KobelShellButtonMode::Iconic => Padding::from([8, 8]),
            KobelShellButtonMode::Text => Padding::from([0, 8]),
            KobelShellButtonMode::IconicAndText => Padding::from([0, 12]),
            KobelShellButtonMode::MenuItem => Padding::from([8, 12]),
        };

        let button_width = match self.mode {
            KobelShellButtonMode::Iconic => iced::Length::Shrink,
            KobelShellButtonMode::Text => iced::Length::Shrink,
            KobelShellButtonMode::IconicAndText => iced::Length::Shrink,
            KobelShellButtonMode::MenuItem => iced::Length::Fill,
        };

        let button_height = match self.mode {
            KobelShellButtonMode::Iconic => iced::Length::Shrink,
            KobelShellButtonMode::Text => iced::Length::Fill,
            KobelShellButtonMode::IconicAndText => iced::Length::Fill,
            KobelShellButtonMode::MenuItem => iced::Length::Shrink,
        };

        let button_align_x = match self.mode {
            KobelShellButtonMode::Iconic => iced::Alignment::Center,
            KobelShellButtonMode::Text => iced::Alignment::Center,
            KobelShellButtonMode::IconicAndText => iced::Alignment::Center,
            KobelShellButtonMode::MenuItem => iced::Alignment::Start,
        };

        let button_container = container(self.content)
            .width(button_width)
            .height(button_height)
            .align_x(button_align_x)
            .align_y(iced::Alignment::Center)
            .style(move |theme: &Theme| {
                container::Style {
                    border: debug_border_style_or_default(&self.state, iced::Border::default()),
                    ..Default::default()
                }
            });

        let button_background = match self.button_type {
            KobelShellButtonType::Normal => Background::Color(Color::TRANSPARENT),
            KobelShellButtonType::Primary => Background::Color(self.state.shell_accent_color),
        };

        let button_hovered_background = match self.button_type {
            KobelShellButtonType::Normal => Background::Color(Color::from_rgba(0.5, 0.5, 0.5, 0.1)),
            KobelShellButtonType::Primary => Background::Color(self.state.shell_accent_color.scale_alpha(0.95)),
        };

        let button_pressed_background = match self.button_type {
            KobelShellButtonType::Normal => Background::Color(Color::from_rgba(0.5, 0.5, 0.5, 0.2)),
            KobelShellButtonType::Primary => Background::Color(self.state.shell_accent_color.scale_alpha(0.85)),
        };

        let button_text_color = match self.button_type {
            KobelShellButtonType::Normal => self.state.shell_text_color,
            KobelShellButtonType::Primary => Color::WHITE,
        };

        button(button_container)
            .width(button_width)
            .height(button_height)
            .padding(button_padding)
            .style(move |theme: &Theme, status| {
                button::Style {
                    background: match status {
                        crate::widget::primitives::button::Status::Pressed => Some(button_pressed_background),
                        crate::widget::primitives::button::Status::Hovered => Some(button_hovered_background),
                        _ => Some(button_background),
                    },
                    text_color: button_text_color,
                    border: debug_border_style_or_default(&self.state, iced::Border {
                        radius: self.radii.into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                }
            })
            .on_press(self.on_press)
            .into()
    }
}

pub fn k_button<'a>(
    state: &'a Arc<KobelShellState>,
    content: impl Into<Element<'a, KobelRootMessage, iced::Theme, iced::Renderer>>,
) -> KobelShellButton<'a>
{
    KobelShellButton::new(state, content)
}
