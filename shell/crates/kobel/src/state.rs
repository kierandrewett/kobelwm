use std::{path::PathBuf, sync::RwLock};

use chrono::{DateTime, Local};
use iced::{font::Family, Background, Color, Font, Task};

use crate::{fps::FpsCounter, panel::bar::{BAR_DEFAULT_HEIGHT, BAR_DEFAULT_MARGIN, BAR_DEFAULT_RADII}, KobelRootMessage};

#[derive(Debug)]
pub struct KobelShellState {
    pub now: RwLock<DateTime<Local>>,
    pub fps: RwLock<FpsCounter>,

    pub font: Font,
    pub font_bold: Font,

    pub shell_background: iced::Background,
    pub shell_text_color: Color,
    pub shell_wallpaper_name: String,

    pub bar_height: i32,
    pub bar_margin: i32,
    pub bar_radii: f32,
    pub bar_radii_bottom_only: bool,

    pub button_radii: f32,
    pub font_base_size: f32,
}

impl KobelShellState {
    pub fn new() -> Self {
        let button_radii = BAR_DEFAULT_RADII - 2.0;

        Self {
            fps: RwLock::new(FpsCounter::new()),
            now: RwLock::new(Local::now()),

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
            shell_wallpaper_name: "wallpaper_light.jpg".to_string(),

            bar_height: BAR_DEFAULT_HEIGHT,
            bar_margin: BAR_DEFAULT_MARGIN,
            bar_radii: BAR_DEFAULT_RADII,
            bar_radii_bottom_only: false,

            button_radii,
            font_base_size: 14.6666,
        }
    }

    pub fn update(&self, message: KobelRootMessage) -> Task<KobelRootMessage> {
        self.fps.write().unwrap().tick();

        let now = Local::now();
        *self.now.write().unwrap() = now;

        Task::none()
    }

    pub fn get_resource_path(&self, name: &str) -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("resources")
            .join(name)
    }
}