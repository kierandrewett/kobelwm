use std::{path::PathBuf, sync::Arc};

use iced::{core::window, platform_specific::shell::commands::{layer_surface::get_layer_surface, subsurface::{Anchor, KeyboardInteractivity, Layer}}, widget::{checkbox, column, container, horizontal_rule, row, slider, svg, text, text_input, tooltip, vertical_rule}, Background, Color, Element, Shadow, Task};
use iced_runtime::platform_specific::wayland::layer_surface::{IcedMargin, SctkLayerSurfaceSettings};

use crate::{state::KobelShellState, widget::{k_text::k_text, primitives::button}, KobelRootMessage};

#[derive(Debug, Clone)]
pub enum KobelDebugMessage {
    Toggle,
    DebugBorderStyleToggled(bool),
}

impl Into<KobelRootMessage> for KobelDebugMessage {
    fn into(self) -> KobelRootMessage {
        KobelRootMessage::Panel(crate::panel::KobelPanelMessage::Debug(self))
    }
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
            anchor: Anchor::TOP | Anchor::RIGHT,
            size: Some((Some(400), Some(400))),
            margin: IcedMargin {
                top: 0,
                left: 10,
                right: 10,
                bottom: 10,
            },
            keyboard_interactivity: KeyboardInteractivity::None,
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
        match message {
            KobelRootMessage::Panel(crate::panel::KobelPanelMessage::Debug(debug_message)) => {
                match debug_message {
                    KobelDebugMessage::Toggle => {
                        let is_visible = self.is_visible();
                    
                        *self.state.debug_panel_visible.write().unwrap() = !is_visible;
                    }
                    KobelDebugMessage::DebugBorderStyleToggled(enabled) => {
                        *self.state.debug_border_style.write().unwrap() = enabled;
                    }
                }
            }
            _ => {}
        }

        Task::none()
    }

    pub fn is_visible(&self) -> bool {
        *self.state.debug_panel_visible.read().unwrap()
    }

    pub fn view(&self) -> Element<KobelRootMessage> {
        if !self.is_visible() {
            return row![].into();
        }

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
            horizontal_rule(2.0),
            k_text(&self.state, format!("Mouse Position: {:?}", self.state.pointer_position.read().unwrap())),
            k_text(&self.state, format!("Screen Size: {:?}", self.state.screen_size.read().unwrap())),
            horizontal_rule(2.0),
            checkbox("Enable debug borders", *self.state.debug_border_style.read().unwrap())
                .on_toggle(|enabled| {
                    KobelRootMessage::Panel(crate::panel::KobelPanelMessage::Debug(KobelDebugMessage::DebugBorderStyleToggled(enabled)))
                })
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