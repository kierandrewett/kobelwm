use std::{path::PathBuf, sync::RwLock};

use chrono::{DateTime, Local};
use iced::{font::Family, keyboard, Background, Color, Font, Task};

use crate::{fps::FpsCounter, panel::{bar::{BAR_DEFAULT_HEIGHT, BAR_DEFAULT_MARGIN, BAR_DEFAULT_PADDING, BAR_DEFAULT_RADII}, dock::{DOCK_DEFAULT_HEIGHT, DOCK_DEFAULT_MARGIN, DOCK_DEFAULT_PADDING, DOCK_DEFAULT_RADII}, search::{SEARCH_DEFAULT_HEIGHT, SEARCH_DEFAULT_MARGIN, SEARCH_DEFAULT_PADDING, SEARCH_DEFAULT_RADII}}, KobelRootMessage};

#[derive(Debug)]
pub struct KobelShellState {
    pub now: RwLock<DateTime<Local>>,
    pub fps: RwLock<FpsCounter>,

    pub screen_size: RwLock<iced::Size>,
    pub pointer_position: RwLock<iced::Point>,

    pub modifiers_pressed: RwLock<keyboard::Modifiers>,
    pub keys_pressed: RwLock<Vec<keyboard::Key>>,

    pub debug_panel_visible: RwLock<bool>,
    pub debug_border_style: RwLock<bool>,

    pub font: Font,
    pub font_bold: Font,

    pub shell_background: iced::Background,
    pub shell_text_color: Color,
    pub shell_accent_color: Color,
    pub shell_wallpaper_name: String,

    pub bar_height: i32,
    pub bar_margin: i32,
    pub bar_radii: f32,
    pub bar_padding: f32,
    pub bar_radii_bottom_only: bool,

    pub font_base_size: f32,
    pub icon_base_size: f32,

    pub dock_height: i32,
    pub dock_margin: i32,
    pub dock_padding: f32,
    pub dock_radii: f32,

    pub search_panel_visible: RwLock<bool>,
    pub search_height: i32,
    pub search_margin: f32,
    pub search_padding: f32,
    pub search_radii: f32,
}

impl KobelShellState {
    pub fn new() -> Self {
        Self {
            fps: RwLock::new(FpsCounter::new()),
            now: RwLock::new(Local::now()),

            screen_size: RwLock::new(iced::Size::default()),
            pointer_position: RwLock::new(iced::Point::default()),

            modifiers_pressed: RwLock::new(keyboard::Modifiers::default()),
            keys_pressed: RwLock::new(Vec::new()),

            debug_panel_visible: RwLock::new(false),
            debug_border_style: RwLock::new(false),

            font: Font {
                family: Family::Name("Cantarell"),
                weight: iced::font::Weight::Normal,
                stretch: iced::font::Stretch::Normal,
                style: iced::font::Style::Normal,
            },
            
            font_bold: Font {
                family: Family::Name("Cantarell"),
                weight: iced::font::Weight::Bold,
                stretch: iced::font::Stretch::Normal,
                style: iced::font::Style::Normal,
            },

            // shell_background: Background::Color(Color::from_rgba(0.05, 0.05, 0.05, 0.95)),
            // shell_text_color: Color::WHITE,
            shell_background: Background::Color(Color::from_rgba(0.95, 0.95, 0.95, 0.925)),
            shell_text_color: Color::BLACK,
            shell_accent_color: Color::from_rgb(0.2078, 0.5176, 0.8941),
            shell_wallpaper_name: "wallpaper_light.jpg".to_string(),

            bar_height: BAR_DEFAULT_HEIGHT,
            bar_margin: BAR_DEFAULT_MARGIN,
            bar_radii: BAR_DEFAULT_RADII,
            bar_padding: BAR_DEFAULT_PADDING,
            bar_radii_bottom_only: false,

            font_base_size: 14.6666,
            icon_base_size: 16.0,

            dock_height: DOCK_DEFAULT_HEIGHT,
            dock_margin: DOCK_DEFAULT_MARGIN,
            dock_padding: DOCK_DEFAULT_PADDING,
            dock_radii: DOCK_DEFAULT_RADII,

            search_panel_visible: RwLock::new(false),
            search_height: SEARCH_DEFAULT_HEIGHT,
            search_margin: SEARCH_DEFAULT_MARGIN,
            search_padding: SEARCH_DEFAULT_PADDING,
            search_radii: SEARCH_DEFAULT_RADII,
        }
    }

    pub fn update(&self, message: KobelRootMessage) -> Task<KobelRootMessage> {
        self.fps.write().unwrap().tick();

        let now = Local::now();
        *self.now.write().unwrap() = now;

        match message {
            KobelRootMessage::CursorMoved { position } => {
                *self.pointer_position.write().unwrap() = position;
            },
            KobelRootMessage::ScreenSizeChanged { size } => {
                *self.screen_size.write().unwrap() = size;
            },
            KobelRootMessage::KeysPressed { modifiers, keys } => {
                let mut keys_pressed = self.keys_pressed.write().unwrap();
                for key in keys {
                    if !keys_pressed.contains(&key) {
                        keys_pressed.push(key);
                    }
                }
                *self.modifiers_pressed.write().unwrap() = modifiers;
            },
            KobelRootMessage::KeysReleased { modifiers, keys } => {
                let mut keys_pressed = self.keys_pressed.write().unwrap();
                for key in keys {
                    keys_pressed.retain(|k| *k != key);
                }
                *self.modifiers_pressed.write().unwrap() = modifiers;
            },
            _ => {}
        }

        Task::none()
    }

    pub fn get_resource_path(&self, name: &str) -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("resources")
            .join(name)
    }
}