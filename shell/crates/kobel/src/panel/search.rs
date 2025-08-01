use std::{path::PathBuf, sync::{Arc, RwLock}};

use iced::{advanced::graphics::text::cache::Key, core::{text::LineHeight, window}, platform_specific::shell::commands::{layer_surface::{destroy_layer_surface, get_layer_surface, set_keyboard_interactivity, set_margin, set_size}, subsurface::{Anchor, KeyboardInteractivity, Layer}}, widget::{column, container, row, svg, text, text_input, tooltip, vertical_rule}, Background, Color, Element, Padding, Task};
use iced_runtime::platform_specific::wayland::layer_surface::{IcedMargin, SctkLayerSurfaceSettings};

use crate::{panel::dock, state::KobelShellState, util::debug::debug_border_style_or_default, widget::{k_button::{k_button, KobelShellButtonMode}, k_icon::k_icon, k_text::k_text, primitives::button}, KobelRootMessage};

pub static SEARCH_DEFAULT_HEIGHT: i32 = 48;
pub static SEARCH_DEFAULT_MARGIN: f32 = 0.25;
pub static SEARCH_DEFAULT_PADDING: f32 = 16.0;
pub static SEARCH_DEFAULT_RADII: f32 = 16.0;

#[derive(Debug, Clone)]
pub enum KobelSearchMessage {
    Toggle,
    QueryUpdated(String),
}

impl Into<KobelRootMessage> for KobelSearchMessage {
    fn into(self) -> KobelRootMessage {
        KobelRootMessage::Panel(crate::panel::KobelPanelMessage::Search(self))
    }
}

pub struct KobelSearch {
    pub id: window::Id,
    state: Arc<KobelShellState>,
    query: String,
}

impl KobelSearch {
    pub fn new(state: Arc<KobelShellState>) -> (Self, Task<KobelRootMessage>) {
        let id = window::Id::unique();

        let search_width = state.screen_size.read().unwrap().width * 0.25;
        let search_margin = state.screen_size.read().unwrap().height * state.search_margin;    

        let surface: Task<KobelRootMessage> = get_layer_surface(SctkLayerSurfaceSettings {
            id,
            namespace: "kobelwm".to_string(),
            layer: Layer::Top,
            anchor: Anchor::TOP,
            size: Some((Some(search_width as u32), Some(state.search_height as u32))),
            exclusive_zone: -1,
            margin: IcedMargin {
                top: search_margin as i32,
                bottom: 0,
                left: 0,
                right: 0,
            },
            keyboard_interactivity: KeyboardInteractivity::None,
            pointer_interactivity: true,
            ..Default::default()
        });

        (
            Self {
                id,
                state,
                query: String::new(),
            },
            surface
        )
    }

    pub fn recompute_search_bounds(state: &Arc<KobelShellState>, id: window::Id, visible: bool) -> Task<KobelRootMessage> {
        let computed_search_width = state.screen_size.read().unwrap().width * 0.25;
        let computed_search_margin = state.screen_size.read().unwrap().height * state.search_margin;

        let search_width = if visible { computed_search_width } else { 1.0 };
        let search_height = if visible { state.search_height as f32 } else { 1.0 };
        let search_margin = if visible { computed_search_margin } else { 0.0 };

        log::warn!("Recomputing search bounds: width={}, height={}, margin={}", search_width, search_height, search_margin);

        Task::batch(vec![
            set_size(id, Some(search_width as u32), Some(search_height as u32)),
            set_margin(id, search_margin as i32, 0, 0, 0),
            set_keyboard_interactivity(id, if visible { KeyboardInteractivity::Exclusive } else { KeyboardInteractivity::None }),
        ])
    }

    pub fn set_visible(&mut self, visible: bool) -> Task<KobelRootMessage> {
        // Only update the surface if the visibility state changes
        if self.is_visible() == visible {
            return Task::none();
        }

        let mut search_visible = self.state.search_panel_visible.write().unwrap();
        *search_visible = visible;

        Self::recompute_search_bounds(&self.state, self.id, visible)
    }

    pub fn is_visible(&self) -> bool {
        *self.state.search_panel_visible.read().unwrap()
    }

    pub fn update(&mut self, message: KobelRootMessage) -> Task<KobelRootMessage> {
        let mut command = Task::none();

        match message {
            KobelRootMessage::KeysReleased { modifiers, keys } => {
                if keys.contains(&iced::keyboard::Key::Named(iced::keyboard::key::Named::Escape)) {
                    command = command.chain(self.set_visible(false));
                } else if keys.contains(&iced::keyboard::Key::Named(iced::keyboard::key::Named::Space)) && modifiers.control() {
                    let is_visible = self.is_visible();
                    command = command.chain(self.set_visible(!is_visible));
                }
            }

            KobelRootMessage::Panel(crate::panel::KobelPanelMessage::Search(message)) => match message {
                KobelSearchMessage::Toggle => {
                    let is_visible = self.is_visible();
                    command = command.chain(self.set_visible(!is_visible));
                },
                KobelSearchMessage::QueryUpdated(query) => {
                    log::warn!("Search query updated: {}", query);
                    self.query = query;
                }
            },
            _ => {}
        }

        command
    }

    pub fn view(&self) -> Element<KobelRootMessage> {
        if !*self.state.search_panel_visible.read().unwrap() {
            return row![].into();
        }

        let search_ui = row![
            k_icon(&self.state, "search.svg"),
            text_input("Type to search this computer", &self.query)
                .on_input(|query| KobelSearchMessage::QueryUpdated(query).into())
                .padding(0)
                .width(iced::Length::Fill)
        ]
            .spacing(self.state.search_padding * 0.75)
            .align_y(iced::Alignment::Center)
            .height(iced::Length::Fill);

        container(container(search_ui)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .align_x(iced::Alignment::Start)
            .align_y(iced::Alignment::Center)
            .padding(Padding::from([0.0, self.state.search_padding]))
            .style(move |_| container::Style {
                background: Some(self.state.shell_background.clone()),
                text_color: Some(self.state.shell_text_color),
                border: debug_border_style_or_default(&self.state, iced::Border {
                    radius: self.state.search_radii.into(),
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