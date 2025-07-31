use std::{path::PathBuf, sync::Arc};

use iced::{core::window, platform_specific::shell::commands::{layer_surface::get_layer_surface, subsurface::{Anchor, KeyboardInteractivity, Layer}}, widget::{column, container, horizontal_rule, row, svg, text, tooltip, vertical_rule}, Background, Color, Element, Shadow, Task};
use iced_runtime::platform_specific::wayland::layer_surface::{IcedMargin, SctkLayerSurfaceSettings};

use crate::{state::KobelShellState, widget::{k_text::k_text, primitives::button}, KobelRootMessage};

#[derive(Debug, Clone)]
pub enum KobelDebugMessage {

}

#[derive(Debug)]
pub struct KobelDebug {
    pub id: window::Id,
    state: Arc<KobelShellState>,
}

impl KobelDebug {
    pub fn new(state: Arc<KobelShellState>) -> (Self, Task<KobelRootMessage>) {
        let id = window::Id::unique();

        let surface = get_layer_surface(SctkLayerSurfaceSettings {
            id,
            namespace: "kobelwm".to_string(),
            layer: Layer::Overlay,
            anchor: Anchor::TOP | Anchor::LEFT,
            size: Some((Some(400), Some(400))),
            margin: IcedMargin {
                top: 0,
                left: 10,
                right: 10,
                bottom: 10,
            },
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
        let wm_info = format!(
            "KobelWM v{}",
            env!("CARGO_PKG_VERSION")
        );

        let sha = env!("VERGEN_GIT_SHA").to_string()[0..7].to_string();

        let debug_ui = column![
            k_text(&self.state, wm_info)
                .bold(true)
                .size(1.5),
            horizontal_rule(2.0),
            k_text(&self.state, format!("Commit: {}", sha)),
            k_text(&self.state, format!("Branch: {}", env!("VERGEN_GIT_BRANCH"))),
            k_text(&self.state, format!("Build Date: {}", env!("VERGEN_BUILD_DATE"))),
            k_text(&self.state, format!("Build Time: {}", env!("VERGEN_BUILD_TIMESTAMP"))),
            k_text(&self.state, format!("Rust Version: {}", env!("VERGEN_RUSTC_SEMVER"))),
            k_text(&self.state, format!("Rust Channel: {}", env!("VERGEN_RUSTC_CHANNEL"))),
            k_text(&self.state, format!("Rust Target: {}", env!("VERGEN_RUSTC_HOST_TRIPLE"))),
            k_text(&self.state, format!("OS: {}", env!("VERGEN_SYSINFO_NAME"))),
            horizontal_rule(2.0),
            k_text(&self.state, format!("FPS: {:.2}", self.state.fps.read().unwrap().fps())),
            k_text(&self.state, format!("Now: {}", self.state.now.read().unwrap().format("%Y-%m-%d %H:%M:%S"))),
        ];

        container(container(debug_ui)
            .padding(6)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .style(move |_| container::Style {
                background: Some(self.state.shell_background.clone()),
                text_color: Some(self.state.shell_text_color),
                border: iced::Border {
                    radius: 10.0.into(),
                    ..Default::default()
                },
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