use std::{path::PathBuf, sync::Arc};

use iced::{core::window, platform_specific::shell::commands::{layer_surface::get_layer_surface, subsurface::{Anchor, KeyboardInteractivity, Layer}}, widget::{column, container, row, svg, text, tooltip, vertical_rule}, Background, Color, Element, Task};
use iced_runtime::platform_specific::wayland::layer_surface::SctkLayerSurfaceSettings;

use crate::{state::KobelShellState, widget::primitives::button, KobelRootMessage};

#[derive(Debug, Clone)]
pub enum KobelDockMessage {

}

#[derive(Debug)]
pub struct KobelDock {
    pub id: window::Id,
    state: Arc<KobelShellState>,
}

impl KobelDock {
    pub fn new(state: Arc<KobelShellState>) -> (Self, Task<KobelRootMessage>) {
        let id = window::Id::unique();

        let surface = get_layer_surface(SctkLayerSurfaceSettings {
            id,
            namespace: "kobelwm".to_string(),
            layer: Layer::Overlay,
            anchor: Anchor::BOTTOM | Anchor::LEFT | Anchor::RIGHT,
            size: Some((None, Some(90))),
            exclusive_zone: 90,
            keyboard_interactivity: KeyboardInteractivity::Exclusive,
            pointer_interactivity: true,
            ..Default::default()
        });

        (
            Self {
                id,
                state,
            },
            surface
        )
    }

    pub fn update(&mut self, message: KobelRootMessage) -> Task<KobelRootMessage> {
        Task::none()
    }

    pub fn view(&self) -> Element<KobelRootMessage> {
        let dock_radii = 34.0;
        let dock_padding = 8;
        let dock_button_radii = dock_radii - dock_padding as f32;
        let dock_button_icon_padding = 14;

        let icons = vec![
            "/usr/share/icons/hicolor/scalable/apps/org.gnome.Nautilus.svg",
            "/usr/share/icons/hicolor/128x128/apps/firefox-nightly.png",
            "/home/kieran/Downloads/discord.png",
            "/usr/share/icons/hicolor/128x128/apps/spotify-client.png",
            "/usr/share/pixmaps/vscode.png",
            "/usr/share/icons/hicolor/scalable/apps/org.gnome.Console.svg",
            "/usr/share/icons/hicolor/scalable/apps/org.gnome.SystemMonitor.svg",
            "/usr/share/icons/hicolor/scalable/apps/org.gnome.Settings.svg",
        ]
            .into_iter()
            .map(|icon_path| PathBuf::from(icon_path))
            .collect::<Vec<_>>();

        let mut icons_ui = vec![];

        for icon in icons {
            let icon_element: Element<KobelRootMessage> = if icon.extension().map_or(false, |ext| ext == "svg") {
                svg(&icon)
                    .width(iced::Length::Shrink)
                    .height(iced::Length::Shrink)
                    .content_fit(iced::ContentFit::Contain)
                    .into()
            } else {
                iced::widget::image(&icon)
                    .width(iced::Length::Shrink)
                    .height(iced::Length::Shrink)
                    .content_fit(iced::ContentFit::Contain)
                    .into()
            };

            let tooltip_text = icon.file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();

            icons_ui.push(
                tooltip(
                    button(container(icon_element)
                        .width(iced::Length::Shrink)
                        .height(iced::Length::Shrink)
                        .align_y(iced::Alignment::Center)
                    )
                        .width(iced::Length::Shrink)
                        .height(iced::Length::Shrink)
                        .padding(dock_button_icon_padding)
                        .style(move |_, status| button::Style {
                            background: match status {
                                crate::widget::primitives::button::Status::Pressed => Some(Background::Color(Color::from_rgba(0.5, 0.5, 0.5, 0.2))),
                                crate::widget::primitives::button::Status::Hovered => Some(Background::Color(Color::from_rgba(0.5, 0.5, 0.5, 0.1))),
                                _ => Some(Background::Color(Color::TRANSPARENT)),
                            },
                            border: iced::Border {
                                width: 0.0,
                                radius: dock_button_radii.into(),
                                color: Color::TRANSPARENT,
                            },
                            ..Default::default()
                        })
                        .on_press(KobelRootMessage::Test),
                        text(tooltip_text)
                            .font(self.state.font_bold)
                            .size(14)
                            .color(Color::WHITE),
                    tooltip::Position::FollowCursor
                )
            );
        }

        let mut dock_ui = row![
            
        ]
            .spacing(4)
            .width(iced::Length::Shrink)
            .height(iced::Length::Shrink);

        for icon_ui in icons_ui {
            dock_ui = dock_ui.push(icon_ui);
        }

        dock_ui = dock_ui.push(vertical_rule(1.0)
            .style(|_| iced::widget::rule::Style {
                fill_mode: iced::widget::rule::FillMode::Full,
                color: Color::from_rgba(0.5, 0.5, 0.5, 0.025),
                width: 1,
                radius: 0.into(),
            })
        );

        dock_ui = dock_ui.push(
            container(
                column![
                    button(text("Kieran")
                        .font(self.state.font_bold)
                        .size(14))
                        .width(iced::Length::Shrink)
                        .height(iced::Length::Shrink)
                        .padding(dock_button_icon_padding)
                        .style(move |_, status| button::Style {
                            background: match status {
                                crate::widget::primitives::button::Status::Pressed => Some(Background::Color(Color::from_rgb(0.2, 0.2, 0.2))),
                                crate::widget::primitives::button::Status::Hovered => Some(Background::Color(Color::from_rgba(0.5, 0.5, 0.5, 0.05))),
                                _ => Some(Background::Color(Color::TRANSPARENT)),
                            },
                            text_color: Color::WHITE,
                            border: iced::Border {
                                width: 0.0,
                                radius: dock_button_radii.into(),
                                color: Color::TRANSPARENT,
                            },
                            ..Default::default()
                        })
                        .on_press(KobelRootMessage::Test),
                    button(text("bingus")
                        .font(self.state.font_bold)
                        .size(14))
                        .width(iced::Length::Shrink)
                        .height(iced::Length::Shrink)
                        .padding(dock_button_icon_padding)
                        .style(move |_, status| button::Style {
                            background: match status {
                                crate::widget::primitives::button::Status::Pressed => Some(Background::Color(Color::from_rgb(0.2, 0.2, 0.2))),
                                crate::widget::primitives::button::Status::Hovered => Some(Background::Color(Color::from_rgba(0.5, 0.5, 0.5, 0.05))),
                                _ => Some(Background::Color(Color::TRANSPARENT)),
                            },
                            text_color: Color::WHITE,
                            border: iced::Border {
                                width: 0.0,
                                radius: dock_button_radii.into(),
                                color: Color::TRANSPARENT,
                            },
                            ..Default::default()
                        })
                        .on_press(KobelRootMessage::Test)
                ]   
            ),
        );

        container(column![container(row![dock_ui])
            .padding(dock_padding)
            .width(iced::Length::Shrink)
            .height(iced::Length::Fill)
            .align_x(iced::Alignment::Center)
            .align_y(iced::Alignment::Center)
            .style(move |_| container::Style {
                background: Some(self.state.shell_background.clone()),
                text_color: Some(Color::WHITE),
                border: iced::Border {
                    width: 3.0,
                    radius: dock_radii.into(),
                    ..Default::default()
                },
                ..container::Style::default()
            })]
        )
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .align_x(iced::Alignment::Center)
            .align_y(iced::Alignment::Center)
            .into()
    }
}