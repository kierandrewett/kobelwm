use std::{path::PathBuf, sync::RwLock};

use chrono::{DateTime, Local};
use iced::{font::Family, Background, Color, Font, Task};

use crate::{fps::FpsCounter, KobelRootMessage};

#[derive(Debug)]
pub struct KobelShellState {
    pub now: RwLock<DateTime<Local>>,
    pub fps: RwLock<FpsCounter>,

    pub font: Font,
    pub font_bold: Font,

    pub shell_background: iced::Background,
    pub shell_text_color: Color,
    pub shell_wallpaper_name: String,
}

impl KobelShellState {
    pub fn new() -> Self {
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
                family: Family::Name("Inter Variable"),
                weight: iced::font::Weight::Medium,
                stretch: iced::font::Stretch::Normal,
                style: iced::font::Style::Normal,
            },

            // shell_background: Background::Color(Color::from_rgba(0.05, 0.05, 0.05, 0.95)),
            // shell_text_color: Color::WHITE,
            shell_background: Background::Color(Color::from_rgba(0.95, 0.95, 0.95, 0.925)),
            shell_text_color: Color::BLACK,
            shell_wallpaper_name: "wallpaper_light.jpg".to_string(),
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