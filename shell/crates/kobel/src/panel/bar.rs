use std::sync::Arc;

use iced::{core::Element, platform_specific::shell::commands::{layer_surface::get_layer_surface, subsurface::{Anchor, KeyboardInteractivity, Layer}}, widget::Row, Background, Color, Padding, Radius, Task};
use iced_runtime::platform_specific::wayland::layer_surface::{IcedMargin, SctkLayerSurfaceSettings};
use iced::widget::{container, row, column, text, svg};
use crate::{widget::primitives::button, KobelRootMessage};

use crate::{state::KobelShellState};

static BAR_DEFAULT_HEIGHT: i32 = 36;

#[derive(Debug, Clone)]
pub enum KobelBarMessage {

}

#[derive(Debug)]
pub struct KobelBar {
    pub id: iced::window::Id,
    state: Arc<KobelShellState>,
}

impl KobelBar {
    pub fn new(state: Arc<KobelShellState>) -> (Self, Task<KobelRootMessage>) {
        let id = iced::window::Id::unique();

        let bar_margin: i32 = 2;
        let bar_height = BAR_DEFAULT_HEIGHT + (bar_margin * 2);

        let surface = get_layer_surface(SctkLayerSurfaceSettings {
            id,
            namespace: "kobelwm".to_string(),
            layer: Layer::Overlay,
            anchor: Anchor::TOP | Anchor::LEFT | Anchor::RIGHT,
            size: Some((None, Some(BAR_DEFAULT_HEIGHT as u32))),
            exclusive_zone: bar_height,
            margin: IcedMargin {
                top: bar_margin,
                bottom: bar_margin,
                left: bar_margin,
                right: bar_margin,
            },
            keyboard_interactivity: KeyboardInteractivity::Exclusive,
            pointer_interactivity: true,
            ..Default::default()
        });

        (
            KobelBar {
                id,
                state,
            },
            surface
        )
    }

    pub fn update(&mut self, message: KobelRootMessage) -> Task<KobelRootMessage> {
        Task::none()
    }

    pub fn view(&self) -> Element<KobelRootMessage, iced::Theme, iced::Renderer> {
        let bar_radii = 14.0;
        let button_radii = bar_radii - 4.0;
        let bar_radii_bottom_only = false;

        let time_str = self.state.now
            .read()
            .unwrap()
            .format("%d %b %H:%M:%S")
            .to_string();

        let info_str = format!(
            "kobelwm v{} ({} fps)",
            env!("CARGO_PKG_VERSION"),
            self.state.fps.read().unwrap().fps() as u32
        );

        let left_ui = column![
            row![
                button(container(text("⬤")
                    .align_y(iced::Alignment::End)
                    .font(self.state.font_bold)
                    .size(14)
                )
                    .align_y(iced::Alignment::Center)
                    .height(iced::Length::Fill)
                )
                    .width(iced::Length::Shrink)
                    .height(iced::Length::Fill)
                    .padding(Padding::from([0, 14]))
                    .style(move |_, status| button::Style {
                        background: match status {
                            crate::widget::primitives::button::Status::Pressed => Some(Background::Color(Color::from_rgba(0.5, 0.5, 0.5, 0.2))),
                            crate::widget::primitives::button::Status::Hovered => Some(Background::Color(Color::from_rgba(0.5, 0.5, 0.5, 0.1))),
                            _ => Some(Background::Color(Color::TRANSPARENT)),
                        },
                        text_color: Color::WHITE,
                        border: iced::Border {
                            width: 0.0,
                            radius: button_radii.into(),
                            color: Color::TRANSPARENT,
                        },
                        ..Default::default()
                    })
                    .on_press(KobelRootMessage::Test),
                text(info_str)
                    .font(self.state.font_bold)
                    .size(14)
                    .align_x(iced::Alignment::Center),
            ]
                .spacing(8)
                .align_y(iced::Alignment::Center)
        ]
            .align_x(iced::Alignment::Start)
            .width(iced::Length::Fill);

        let clock_ui = column![
            button(container(text(time_str)
                .font(self.state.font_bold)
                .size(14)
                .width(iced::Length::Shrink)
                .height(iced::Length::Fill)
                .align_y(iced::Alignment::Center))
            )
                .width(iced::Length::Shrink)
                .height(iced::Length::Fill)
                .padding(Padding::from([0, 12]))
                .style(move |_, status| button::Style {
                    background: match status {
                        crate::widget::primitives::button::Status::Pressed => Some(Background::Color(Color::from_rgba(0.5, 0.5, 0.5, 0.2))),
                        crate::widget::primitives::button::Status::Hovered => Some(Background::Color(Color::from_rgba(0.5, 0.5, 0.5, 0.1))),
                        _ => Some(Background::Color(Color::TRANSPARENT)),
                    },
                    text_color: Color::WHITE,
                    border: iced::Border {
                        width: 0.0,
                        radius: button_radii.into(),
                        color: Color::TRANSPARENT,
                    },
                    ..Default::default()
                })
                .on_press(KobelRootMessage::Test)
        ]
        .spacing(8)
        .align_x(iced::Alignment::Center)
        .width(iced::Length::Fill);

        let action_icons = vec![
            "devices.svg",
            "network_wired.svg",
            "speaker_100.svg",
            "power.svg",
        ]
            .into_iter()
            .map(|r| self.state.get_resource_path(r))
            .map(|icon_path| svg(&icon_path)
                .width(iced::Length::Fixed(16.0))
                .height(iced::Length::Fixed(16.0))
                .content_fit(iced::ContentFit::Contain)
                .symbolic(true)
                .style(|_, _| svg::Style {
                    color: Some(Color::WHITE),
                    ..Default::default()
                })
                .into())
            .collect::<Vec<_>>();

        let actions_ui= container(row![
            button(container(row![
                    iced::widget::image("/home/kieran/Downloads/barking.jpg")
                        .width(iced::Length::Fixed(24.0))
                        .height(iced::Length::Fixed(24.0))
                        .content_fit(iced::ContentFit::Contain)
                        .border_radius([button_radii, button_radii, button_radii, button_radii]),
                    text("Barking - Ramz")
                        .font(self.state.font_bold)
                        .size(14)
                        .align_y(iced::Alignment::Center)
                ]
                .align_y(iced::Alignment::Center)
                .spacing(8)
            )
                .align_y(iced::Alignment::Center)
                .height(iced::Length::Fill)
            )
                .width(iced::Length::Shrink)
                .height(iced::Length::Fill)
                .padding(Padding::from([0, 14]))
                .style(move |_, status| button::Style {
                    background: match status {
                        crate::widget::primitives::button::Status::Pressed => Some(Background::Color(Color::from_rgba(0.5, 0.5, 0.5, 0.2))),
                        crate::widget::primitives::button::Status::Hovered => Some(Background::Color(Color::from_rgba(0.5, 0.5, 0.5, 0.1))),
                        _ => Some(Background::Color(Color::TRANSPARENT)),
                    },
                    text_color: Color::WHITE,
                    border: iced::Border {
                        width: 0.0,
                        radius: button_radii.into(),
                        color: Color::TRANSPARENT,
                    },
                    ..Default::default()
                })
                .on_press(KobelRootMessage::Test),
            button(container(iced::widget::image("/home/kieran/.config/discordcanary/0.0.721/modules/discord_desktop_core/asar/app/images/systemtray/linux/tray-unread.png")
                .width(iced::Length::Fixed(16.0))
                .height(iced::Length::Fixed(16.0))
                .content_fit(iced::ContentFit::Contain)
            )
                .align_y(iced::Alignment::Center)
                .height(iced::Length::Fill)
            )
                .width(iced::Length::Shrink)
                .height(iced::Length::Fill)
                .padding(Padding::from([0, 14]))
                .style(move |_, status| button::Style {
                    background: match status {
                        crate::widget::primitives::button::Status::Pressed => Some(Background::Color(Color::from_rgba(0.5, 0.5, 0.5, 0.2))),
                        crate::widget::primitives::button::Status::Hovered => Some(Background::Color(Color::from_rgba(0.5, 0.5, 0.5, 0.1))),
                        _ => Some(Background::Color(Color::TRANSPARENT)),
                    },
                    text_color: Color::WHITE,
                    border: iced::Border {
                        width: 0.0,
                        radius: button_radii.into(),
                        color: Color::TRANSPARENT,
                    },
                    ..Default::default()
                })
                .on_press(KobelRootMessage::Test),
            button(container(text("en₁")
                    .align_y(iced::Alignment::End)
                .font(self.state.font_bold)
                .size(14)
            )
                .align_y(iced::Alignment::Center)
                .height(iced::Length::Fill)
            )
                .width(iced::Length::Shrink)
                .height(iced::Length::Fill)
                .padding(Padding::from([0, 14]))
                .style(move |_, status| button::Style {
                    background: match status {
                        crate::widget::primitives::button::Status::Pressed => Some(Background::Color(Color::from_rgba(0.5, 0.5, 0.5, 0.2))),
                        crate::widget::primitives::button::Status::Hovered => Some(Background::Color(Color::from_rgba(0.5, 0.5, 0.5, 0.1))),
                        _ => Some(Background::Color(Color::TRANSPARENT)),
                    },
                    text_color: Color::WHITE,
                    border: iced::Border {
                        width: 0.0,
                        radius: button_radii.into(),
                        color: Color::TRANSPARENT,
                    },
                    ..Default::default()
                })
                .on_press(KobelRootMessage::Test),
            button(container(Row::from_vec(action_icons).spacing(12))
                .align_y(iced::Alignment::Center)
                .height(iced::Length::Fill)
            )
                .width(iced::Length::Shrink)
                .height(iced::Length::Fill)
                .padding(Padding::from([0, 14]))
                .style(move |_, status| button::Style {
                    background: match status {
                        crate::widget::primitives::button::Status::Pressed => Some(Background::Color(Color::from_rgba(0.5, 0.5, 0.5, 0.2))),
                        crate::widget::primitives::button::Status::Hovered => Some(Background::Color(Color::from_rgba(0.5, 0.5, 0.5, 0.1))),
                        _ => Some(Background::Color(Color::TRANSPARENT)),
                    },
                    text_color: Color::WHITE,
                    border: iced::Border {
                        width: 0.0,
                        radius: button_radii.into(),
                        color: Color::TRANSPARENT,
                    },
                    ..Default::default()
                })
                .on_press(KobelRootMessage::Test)
        ]
            .width(iced::Length::Shrink)
            .spacing(4)
        )
            .width(iced::Length::Fill)
            .align_x(iced::Alignment::End)
            .align_y(iced::Alignment::Center);

        let bar_row = row![left_ui, clock_ui, actions_ui]
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .align_y(iced::Alignment::Center)
            .padding(Padding::from([4, 4]));

        container(bar_row)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .align_x(iced::Alignment::Center)
            .align_y(iced::Alignment::Center)
            .style(move |_| container::Style {
                background: Some(self.state.shell_background.clone()),
                text_color: Some(Color::WHITE),
                border: iced::Border {
                    width: 0.0,
                    radius: Radius {
                        top_left: if bar_radii_bottom_only {
                            0.0.into()
                        } else {
                            bar_radii.into()
                        },
                        top_right: if bar_radii_bottom_only {
                            0.0.into()
                        } else {
                            bar_radii.into()
                        },
                        bottom_left: bar_radii.into(),
                        bottom_right: bar_radii.into(),
                    },
                    color: Color::TRANSPARENT,
                },
                ..container::Style::default()
            })
            .into()
    }
}