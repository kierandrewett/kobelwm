mod fps;
mod widget;
mod panel;
mod state;
mod util;

use chrono::{DateTime, Local};

use iced::advanced::subscription;
use iced::daemon::Appearance;
use iced::widget::row;
use iced::window::Id;
use iced::{event, keyboard, mouse, Color, Element, Subscription, Task, Theme};

use std::sync::{Arc};
use std::time::{Duration};

use crate::panel::bar::KobelBar;
use crate::panel::context_menu::KobelContextMenu;
use crate::panel::debug::KobelDebug;
use crate::panel::dock::KobelDock;
use crate::panel::search::{self, KobelSearch};
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

struct App {
    state: Arc<KobelShellState>,

    wallpaper: KobelWallpaper,
    bar: KobelBar,
    dock: KobelDock,
    debug: KobelDebug,
    search: KobelSearch,

    context_menu: Option<KobelContextMenu>,
}

#[derive(Debug, Clone)]
pub enum KobelRootMessage {
    Tick(DateTime<Local>),
    Noop,

    CursorMoved {
        position: iced::Point,
    },
    KeysPressed {
        modifiers: keyboard::Modifiers,
        keys: Vec<keyboard::Key>,
    },
    KeysReleased {
        modifiers: keyboard::Modifiers,
        keys: Vec<keyboard::Key>,
    },
    ScreenSizeChanged {
        size: iced::Size,
    },

    Panel(panel::KobelPanelMessage),

    OpenContextMenu {
        width: f32,
        height: f32,
    },
}

impl App {
    fn new() -> (App, Task<KobelRootMessage>) {
        let state = Arc::new(KobelShellState::new());

        let (wallpaper, wallpaper_task) = KobelWallpaper::new(state.clone());
        let (bar, bar_task) = KobelBar::new(state.clone());
        let (dock, dock_task) = KobelDock::new(state.clone());
        let (debug, debug_task) = KobelDebug::new(state.clone());
        let (search, search_task) = KobelSearch::new(state.clone());

        (
            Self {
                state,

                wallpaper,
                bar,
                dock,
                debug,
                search,

                context_menu: None,
            },
            Task::batch(vec![
                wallpaper_task,
                bar_task,
                dock_task,
                debug_task,
                search_task,
            ]),
        )
    }

    fn title(&self, _id: Id) -> String {
        String::from("kobelwm")
    }

    fn update(&mut self, message: KobelRootMessage) -> Task<KobelRootMessage> {
        let mut command = Task::batch(vec![
            self.state.update(message.clone()),
            self.wallpaper.update(message.clone()),
            self.bar.update(message.clone()),
            self.dock.update(message.clone()),
            self.debug.update(message.clone()),
            self.search.update(message.clone()),
        ]);

        match message {
            KobelRootMessage::Noop => {
                log::warn!("noop");
            },
            KobelRootMessage::OpenContextMenu { width, height } => {
                let rect = iced::Rectangle {
                    x: self.state.pointer_position.read().unwrap().x,
                    y: self.state.pointer_position.read().unwrap().y,
                    width,
                    height,
                };

                let (context_menu, task) = KobelContextMenu::new(self.state.clone(), rect);
                self.context_menu = Some(context_menu);
                command = command.chain(task);
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
            id if id == self.debug.id => self.debug.view(),
            id if id == self.search.id => self.search.view(),
            id if self.context_menu.as_ref().map_or(false, |cm| cm.id == id) => {
                if let Some(context_menu) = &self.context_menu {
                    context_menu.view()
                } else {
                    row![].into()
                }
            },
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
            iced::event::listen_with(|evt, status, window_id| 
                match evt {
                    event::Event::Mouse(mouse::Event::CursorMoved { position }) => {
                        Some(KobelRootMessage::CursorMoved { position })
                    },
                    event::Event::Keyboard(keyboard::Event::KeyPressed { key, modifiers, .. }) => {
                        Some(KobelRootMessage::KeysPressed {
                            modifiers,
                            keys: vec![key],
                        })
                    },
                    event::Event::Keyboard(keyboard::Event::KeyReleased { key, modifiers, .. }) => {
                        Some(KobelRootMessage::KeysReleased {
                            modifiers,
                            keys: vec![key],
                        })
                    },
                    event::Event::PlatformSpecific(event::PlatformSpecific::Wayland(evt)) => {
                        match evt {
                            event::wayland::Event::Output(output_evt, _) => match output_evt {
                                event::wayland::OutputEvent::InfoUpdate(info) => {
                                    Some(KobelRootMessage::ScreenSizeChanged {
                                        size: iced::Size {
                                            width: info.logical_size.unwrap_or_default().0 as f32,
                                            height: info.logical_size.unwrap_or_default().1 as f32,
                                        },
                                    })
                                },
                                _ => None,
                            },
                            _ => {
                                None
                            }
                        }
                    },
                    _ => None,
                }
            ),
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
