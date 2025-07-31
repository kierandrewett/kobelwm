use std::sync::Arc;

use iced::{widget::container, Background, Color, Element, Padding, Theme};

use crate::{state::KobelShellState, widget::primitives::{self, button}, KobelRootMessage};

pub fn k_button<'a>(
    state: &'a Arc<KobelShellState>,
    content: impl Into<Element<'a, KobelRootMessage, iced::Theme, iced::Renderer>>,
) -> primitives::button::Button<'a, KobelRootMessage, iced::Theme, iced::Renderer>
{
    let button_container = container(content)
        .width(iced::Length::Shrink)
        .height(iced::Length::Fill)
        .align_x(iced::Alignment::Center)
        .align_y(iced::Alignment::Center);

    button(button_container)
        .width(iced::Length::Shrink)
        .height(iced::Length::Fill)
        .padding(Padding::from([0, 12]))
        .style(move |theme: &Theme, status| {
            button::Style {
                background: match status {
                    crate::widget::primitives::button::Status::Pressed => Some(Background::Color(Color::from_rgba(0.5, 0.5, 0.5, 0.2))),
                    crate::widget::primitives::button::Status::Hovered => Some(Background::Color(Color::from_rgba(0.5, 0.5, 0.5, 0.1))),
                    _ => Some(Background::Color(Color::TRANSPARENT)),
                },
                text_color: state.shell_text_color,
                border: iced::Border {
                    width: 0.0,
                    radius: state.button_radii.into(),
                    color: Color::TRANSPARENT,
                },
                ..Default::default()
            }
        })
        .on_press(KobelRootMessage::Test)
}