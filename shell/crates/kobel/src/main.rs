mod fps;
mod widget;
mod panel;
mod state;

use chrono::{DateTime, Local, TimeDelta, Utc};

use iced::alignment::Vertical;
use iced::daemon::Appearance;
use iced::font::Family;
use iced::futures::{stream, SinkExt, Stream};
use iced::platform_specific::runtime::wayland::layer_surface::SctkLayerSurfaceSettings;
use iced::platform_specific::shell::commands::layer_surface::{
    Anchor, KeyboardInteractivity, Layer, get_layer_surface,
};
use iced::runtime::platform_specific::wayland::layer_surface::IcedMargin;
use iced::theme::Palette;
use iced::widget::{center, column, container, pick_list, rich_text, row, slider, span, svg, text, tooltip, vertical_rule, Row};
use iced::window::Id;
use iced::Length::FillPortion;
use iced::{border, Background, Color, Element, Font, Padding, Radius, Subscription, Task, Theme};

use std::path::PathBuf;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::{Duration, Instant};

use crate::panel::bar::KobelBar;
use crate::panel::dock::KobelDock;
use crate::panel::wallpaper::KobelWallpaper;
use crate::state::KobelShellState;
use crate::widget::primitives::button;

#[tokio::main]
pub async fn main() -> iced::Result {
    env_logger::init();

    iced::daemon(App::title, App::update, App::view)
        .subscription(App::subscription)
        .theme(App::theme)
        .style(App::style)
        .run_with(App::new)
}

#[derive(Debug)]
struct App {
    state: Arc<KobelShellState>,

    wallpaper: KobelWallpaper,
    bar: KobelBar,
    dock: KobelDock,
}

#[derive(Debug, Clone)]
pub enum KobelRootMessage {
    Tick(DateTime<Local>),
    Test,

    Panel(panel::KobelPanelMessage)
}

impl App {
    fn new() -> (App, Task<KobelRootMessage>) {
        let state = Arc::new(KobelShellState::new());

        let (wallpaper, wallpaper_task) = KobelWallpaper::new(state.clone());
        let (bar, bar_task) = KobelBar::new(state.clone());
        let (dock, dock_task) = KobelDock::new(state.clone());

        (
            Self {
                state,

                wallpaper,
                bar,
                dock,
            },
            Task::batch(vec![
                wallpaper_task,
                bar_task,
                dock_task,
            ]),
        )
    }

    fn title(&self, _id: Id) -> String {
        String::from("kobelwm")
    }

    fn update(&mut self, message: KobelRootMessage) -> Task<KobelRootMessage> {
        let command = Task::batch(vec![
            self.state.update(message.clone()),
            self.wallpaper.update(message.clone()),
            self.bar.update(message.clone()),
            self.dock.update(message.clone()),
        ]);

        match message {
            KobelRootMessage::Test => {
                log::warn!("Test button pressed!");
            },
            _ => {}
        }

        return command;
    }

    fn view(&self, id: Id) -> Element<KobelRootMessage> {
        match id {
            id if id == self.wallpaper.id => self.wallpaper.view(),
            id if id == self.bar.id => self.bar.view(),
            id if id == self.dock.id => self.dock.view(),
            _ => {
                log::warn!("Unknown window ID: {:?}", id);
                row![].into()
            },
        }

    }

    fn subscription(&self) -> Subscription<KobelRootMessage> {
        Subscription::batch(vec![
            iced::time::every(Duration::from_millis(8))
                .map(|_| KobelRootMessage::Tick(Local::now())),
        ])
    }

    fn theme(&self, _id: Id) -> iced::Theme {
        Theme::KanagawaLotus
    }

    fn style(&self, theme: &Theme) -> Appearance {
        Appearance {
            background_color: Color::TRANSPARENT,
            text_color: theme.palette().text,
            icon_color: theme.palette().text,
        }
    }
}
