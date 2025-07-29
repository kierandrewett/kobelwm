mod fps;
mod widget;
mod panel;
mod state;

use chrono::{DateTime, Local};

use iced::daemon::Appearance;
use iced::widget::row;
use iced::window::Id;
use iced::{Color, Element, Subscription, Task, Theme};

use std::sync::{Arc};
use std::time::{Duration};

use crate::panel::bar::KobelBar;
use crate::panel::dock::KobelDock;
use crate::panel::wallpaper::KobelWallpaper;
use crate::state::KobelShellState;

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
