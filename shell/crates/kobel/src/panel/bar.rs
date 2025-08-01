use std::sync::Arc;

use iced::{core::{Element, Widget}, platform_specific::shell::commands::{layer_surface::get_layer_surface, subsurface::{Anchor, KeyboardInteractivity, Layer}}, widget::Row, Background, Color, Padding, Radius, Task};
use iced_runtime::platform_specific::wayland::layer_surface::{IcedMargin, SctkLayerSurfaceSettings};
use iced::widget::{container, row, column, text, svg};
use crate::{widget::{k_button::{k_button, KobelShellButtonType}, k_icon::k_icon, k_text::k_text, primitives::button}, KobelRootMessage};

use crate::{state::KobelShellState};

pub static BAR_DEFAULT_HEIGHT: i32 = 36;
pub static BAR_DEFAULT_MARGIN: i32 = 4;
pub static BAR_DEFAULT_PADDING: f32 = 4.0;
pub static BAR_DEFAULT_RADII: f32 = 14.0;

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

        let surface = get_layer_surface(SctkLayerSurfaceSettings {
            id,
            namespace: "kobelwm".to_string(),
            layer: Layer::Overlay,
            anchor: Anchor::TOP | Anchor::LEFT | Anchor::RIGHT,
            size: Some((None, Some(state.bar_height as u32))),
            exclusive_zone: state.bar_height + (state.bar_margin * 2),
            margin: IcedMargin {
                top: state.bar_margin,
                bottom: state.bar_margin,
                left: state.bar_margin,
                right: state.bar_margin,
            },
            keyboard_interactivity: KeyboardInteractivity::OnDemand,
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
        let button_radii = self.state.bar_radii - self.state.bar_padding;

        let time_str = self.state.now
            .read()
            .unwrap()
            .format("%d %b  %H:%M:%S")
            .to_string();

        let left_ui = column![
            row![
                k_button(&self.state, k_icon(&self.state, "logo.svg"))
                    .radii(button_radii),
            ]
                .spacing(8)
                .align_y(iced::Alignment::Center)
        ]
            .align_x(iced::Alignment::Start)
            .width(iced::Length::Fill);

        let clock_ui = column![
            k_button(&self.state, k_text(&self.state, time_str).bold(true))
                .radii(button_radii)
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
            .map(|r| k_icon(&self.state, r).into())
            .collect::<Vec<_>>();

        let mut actions_row = Row::from_vec(vec![
            k_button(&self.state, row![
                iced::widget::image("/home/kieran/Downloads/barking.jpg")
                    .width(iced::Length::Fixed(24.0))
                    .height(iced::Length::Fixed(24.0))
                    .content_fit(iced::ContentFit::Contain),
                k_text(&self.state, "Barking - Ramz")
                    .bold(true)
            ]
                .spacing(8)
                .align_y(iced::Alignment::Center)
            )
                .radii(button_radii)
                .into(),
            k_button(&self.state, k_icon(&self.state, "/home/kieran/.config/discordcanary/0.0.721/modules/discord_desktop_core/asar/app/images/systemtray/linux/tray-unread.png"))
                .radii(button_radii)
                .into(),
            k_button(&self.state, k_text(&self.state, "en₁").bold(true))
                .radii(button_radii)
                .into(),
            k_button(&self.state, Row::from_vec(action_icons).spacing(12))
                .radii(button_radii)
                .into(),
            k_button(&self.state, k_icon(&self.state, "search.svg")
                .color(if *self.state.search_panel_visible.read().unwrap() {
                    Some(Color::WHITE)
                } else {
                    None
                })
            )
                .radii(button_radii)
                .button_type(if *self.state.search_panel_visible.read().unwrap() {
                    KobelShellButtonType::Primary
                } else {
                    KobelShellButtonType::Normal
                })
                .on_press(KobelRootMessage::Panel(crate::panel::KobelPanelMessage::Debug(crate::panel::debug::KobelDebugMessage::Toggle)))
                .into()
        ]);

        if cfg!(debug_assertions) {
            actions_row = actions_row.push(
                k_button(&self.state, k_icon(&self.state, "inspector.svg")
                    .color(if *self.state.debug_panel_visible.read().unwrap() {
                        Some(Color::WHITE)
                    } else {
                        None
                    })
                )
                    .radii(button_radii)
                    .button_type(if *self.state.debug_panel_visible.read().unwrap() {
                        KobelShellButtonType::Primary
                    } else {
                        KobelShellButtonType::Normal
                    })
                    .on_press(KobelRootMessage::Panel(crate::panel::KobelPanelMessage::Debug(crate::panel::debug::KobelDebugMessage::Toggle)))
            );
        }

        let actions_ui= container(actions_row
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
            .padding(self.state.bar_padding);

        container(bar_row)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .align_x(iced::Alignment::Center)
            .align_y(iced::Alignment::Center)
            .style(move |_| container::Style {
                background: Some(self.state.shell_background.clone()),
                text_color: Some(self.state.shell_text_color),
                border: iced::Border {
                    width: 0.0,
                    radius: Radius {
                        top_left: if self.state.bar_radii_bottom_only {
                            0.0.into()
                        } else {
                            self.state.bar_radii.into()
                        },
                        top_right: if self.state.bar_radii_bottom_only {
                            0.0.into()
                        } else {
                            self.state.bar_radii.into()
                        },
                        bottom_left: self.state.bar_radii.into(),
                        bottom_right: self.state.bar_radii.into(),
                    },
                    color: Color::TRANSPARENT,
                },
                ..container::Style::default()
            })
            .into()
    }
}