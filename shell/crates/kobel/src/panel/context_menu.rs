use std::{path::PathBuf, sync::Arc};

use iced::{core::window, platform_specific::shell::commands::{layer_surface::get_layer_surface, subsurface::{Anchor, KeyboardInteractivity, Layer}}, widget::{checkbox, column, container, horizontal_rule, row, slider, svg, text, text_input, tooltip, vertical_rule}, window::Position, Background, Color, Element, Point, Rectangle, Shadow, Task};
use iced_runtime::platform_specific::wayland::layer_surface::{IcedMargin, SctkLayerSurfaceSettings};

use crate::{state::KobelShellState, widget::{k_button::{k_button, KobelShellButtonMode}, k_text::k_text, primitives::button}, KobelRootMessage};

#[derive(Debug, Clone)]
pub enum KobelContextMenuMessage {
    Toggle,
    DebugBorderStyleToggled(bool),
}

impl Into<KobelRootMessage> for KobelContextMenuMessage {
    fn into(self) -> KobelRootMessage {
        KobelRootMessage::Panel(crate::panel::KobelPanelMessage::ContextMenu(self))
    }
}

#[derive(Debug)]
pub struct KobelContextMenu {
    pub id: window::Id,
    state: Arc<KobelShellState>,
}

impl KobelContextMenu {
    pub fn new(state: Arc<KobelShellState>, rect: Rectangle) -> (Self, Task<KobelRootMessage>) {
        let id = window::Id::unique();

        let surface = get_layer_surface(SctkLayerSurfaceSettings {
            id,
            namespace: "kobelwm".to_string(),
            layer: Layer::Overlay,
            anchor: Anchor::TOP | Anchor::LEFT,
            size: Some((Some(rect.width as u32), Some(rect.height as u32))),
            exclusive_zone: -1,
            margin: IcedMargin {
                top: rect.y as i32,
                left: rect.x as i32,
                right: 0,
                bottom: 0,
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
        let debug_ui = column![
            k_button(&self.state, k_text(&self.state, "Change wallpaper..."))
                .mode(KobelShellButtonMode::MenuItem),
            horizontal_rule(10.0),
            k_button(&self.state, k_text(&self.state, "Display Settings"))
                .mode(KobelShellButtonMode::MenuItem),
            k_button(&self.state, k_text(&self.state, "Settings"))
                .mode(KobelShellButtonMode::MenuItem),
            horizontal_rule(10.0),
            k_button(&self.state, k_text(&self.state, "Lock"))
                .mode(KobelShellButtonMode::MenuItem)
        ]
            .spacing(2);

        container(container(debug_ui)
            .padding(6)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .style(move |_| container::Style {
                background: Some(self.state.shell_background.clone().scale_alpha(2.0)),
                text_color: Some(self.state.shell_text_color),
                border: iced::Border {
                    width: 1.0,
                    color: self.state.shell_text_color.scale_alpha(0.15),
                    radius: 14.0.into(),
                    ..Default::default()
                },
                ..container::Style::default()
            })
        )
            .width(iced::Length::Shrink)
            .height(iced::Length::Shrink)
            .align_x(iced::Alignment::Center)
            .align_y(iced::Alignment::Center)
            .into()
    }
}