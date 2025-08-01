use std::{path::PathBuf, sync::Arc};

use iced::{core::window, platform_specific::shell::commands::{layer_surface::get_layer_surface, subsurface::{Anchor, KeyboardInteractivity, Layer}}, widget::{column, container, row, svg, text, tooltip, vertical_rule}, Background, Color, Element, Task};
use iced_runtime::platform_specific::wayland::layer_surface::{IcedMargin, SctkLayerSurfaceSettings};

use crate::{panel::dock, state::KobelShellState, util::debug::debug_border_style_or_default, widget::{k_button::{k_button, KobelShellButtonMode}, k_icon::k_icon, k_text::k_text, primitives::button}, KobelRootMessage};

pub static DOCK_DEFAULT_HEIGHT: i32 = 84;
pub static DOCK_DEFAULT_MARGIN: i32 = 8;
pub static DOCK_DEFAULT_PADDING: f32 = 6.0;
pub static DOCK_DEFAULT_RADII: f32 = 24.0;

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

        let dock_height = state.dock_height + (state.dock_margin * 2);

        let surface = get_layer_surface(SctkLayerSurfaceSettings {
            id,
            namespace: "kobelwm".to_string(),
            layer: Layer::Overlay,
            anchor: Anchor::BOTTOM | Anchor::LEFT | Anchor::RIGHT,
            size: Some((None, Some(state.dock_height as u32))),
            exclusive_zone: dock_height,
            margin: IcedMargin {
                top: state.dock_margin,
                bottom: state.dock_margin,
                left: state.dock_margin,
                right: state.dock_margin,
            },
            keyboard_interactivity: KeyboardInteractivity::OnDemand,
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
        let icon_size = (self.state.dock_height as f32 - (self.state.dock_padding * 2.0)) / self.state.icon_base_size;
        let button_radii = self.state.dock_radii - self.state.dock_padding;

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
            let icon_element: Element<KobelRootMessage> = k_icon(&self.state, icon.to_string_lossy().to_string())
                .size(iced::Length::Shrink)
                .symbolic(false)
                .into();

            let tooltip_text = icon.file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();

            icons_ui.push(
                tooltip(
                    k_button(&self.state, icon_element)
                        .radii(button_radii)
                        .mode(KobelShellButtonMode::Iconic),
                    k_text(&self.state, tooltip_text),
                    tooltip::Position::FollowCursor
                )
            );
        }

        let mut dock_ui = row![
            
        ]
            .spacing(self.state.dock_padding * 1.5)
            .width(iced::Length::Shrink)
            .height(iced::Length::Shrink);

        for icon_ui in icons_ui {
            dock_ui = dock_ui.push(icon_ui);
        }

        // dock_ui = dock_ui.push(vertical_rule(1.0));

        // dock_ui = dock_ui.push(
        //     row![
        //         k_button(&self.state, k_text(&self.state, "Kieran")),
        //         k_button(&self.state, k_text(&self.state, "Bingus"))
        //     ],
        // );

        container(container(row![dock_ui])
            .width(iced::Length::Shrink)
            .height(iced::Length::Fill)
            .align_x(iced::Alignment::Center)
            .align_y(iced::Alignment::Center)
            .padding(self.state.dock_padding)
            .style(move |_| container::Style {
                background: Some(self.state.shell_background.clone()),
                text_color: Some(self.state.shell_text_color),
                border: debug_border_style_or_default(&self.state, iced::Border {
                    radius: self.state.dock_radii.into(),
                    ..Default::default()
                }),
                ..container::Style::default()
            })
        )
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .align_x(iced::Alignment::Center)
            .align_y(iced::Alignment::Center)
            .into()
    }
}