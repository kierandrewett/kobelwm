use std::sync::Arc;

use iced::{core::window, platform_specific::shell::commands::{layer_surface::get_layer_surface, subsurface::{Anchor, KeyboardInteractivity, Layer}}, widget::{container, row}, window::Id, Background, Color, Element, Task};
use iced_runtime::platform_specific::wayland::layer_surface::SctkLayerSurfaceSettings;

use crate::{state::KobelShellState, KobelRootMessage};

#[derive(Debug, Clone)]
pub enum KobelWallpaperMessage {
    WallpaperLoaded(iced::widget::image::Handle),
    WallpaperFailed,
}

impl Into<KobelRootMessage> for KobelWallpaperMessage {
    fn into(self) -> KobelRootMessage {
        KobelRootMessage::Panel(crate::panel::KobelPanelMessage::Wallpaper(self))
    }
}

#[derive(Debug)]
pub struct KobelWallpaper {
    pub id: window::Id,
    state: Arc<KobelShellState>,

    wallpaper_opacity: f32,
    wallpaper_handle: Option<iced::widget::image::Handle>,
}

impl KobelWallpaper {
    pub fn new(state: Arc<KobelShellState>) -> (Self, Task<KobelRootMessage>) {
        let id = Id::unique();

        let surface = get_layer_surface(SctkLayerSurfaceSettings {
            id,
            namespace: "kobelwm".to_string(),
            layer: Layer::Background,
            anchor: Anchor::TOP | Anchor::LEFT | Anchor::RIGHT | Anchor::BOTTOM,
            size: Some((None, None)),
            exclusive_zone: -1,
            keyboard_interactivity: KeyboardInteractivity::None,
            pointer_interactivity: false,
            ..Default::default()
        });

        (
            Self {
                id,
                state: state.clone(),

                wallpaper_opacity: 0.0,
                wallpaper_handle: None,
            },
            Task::batch(vec![
                surface,
                Task::perform(
                    Self::load_wallpaper(state.clone()),
                    |result| match result {
                        Ok(handle) => KobelWallpaperMessage::WallpaperLoaded(handle).into(),
                        Err(_) => KobelWallpaperMessage::WallpaperFailed.into(),
                    },
                )
            ])
        )
    }

    pub fn update(&mut self, message: KobelRootMessage) -> Task<KobelRootMessage> {
        if let KobelRootMessage::Panel(crate::panel::KobelPanelMessage::Wallpaper(wallpaper_message)) = message {
            match wallpaper_message {
                KobelWallpaperMessage::WallpaperLoaded(handle) => {
                    self.wallpaper_handle = Some(handle);
                    self.wallpaper_opacity = 1.0; // Start with full opacity
                }
                KobelWallpaperMessage::WallpaperFailed => {
                    log::error!("Failed to load wallpaper");
                }
            }
        }

        Task::none()
    }

    pub fn view(&self) -> Element<KobelRootMessage> {
        let wallpaper_element: Element<KobelRootMessage> = if let Some(handle) = &self.wallpaper_handle {
            iced::widget::image(handle)
                .width(iced::Length::Fill)
                .height(iced::Length::Fill)
                .content_fit(iced::ContentFit::Cover)
                .opacity(self.wallpaper_opacity)
                .into()
        } else {
            row![]
                .into()
        };

        container(wallpaper_element)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .align_x(iced::Alignment::Center)
            .align_y(iced::Alignment::Center)
            .style(move |_| container::Style {
                background: Some(Background::Color(Color::BLACK)),
                ..container::Style::default()
            })
            .into()
    }

    async fn load_wallpaper(state: Arc<KobelShellState>) -> anyhow::Result<iced::widget::image::Handle> {
        log::info!("Loading wallpaper...");
                
        let path = state.get_resource_path("wallpaper.jpg");
        if !path.exists() {
            log::error!("Wallpaper not found: {}", path.display());
            anyhow::bail!("Wallpaper not found");
        }

        // load and decode in background thread
        let handle = tokio::task::spawn_blocking(move || -> anyhow::Result<iced::widget::image::Handle> {
            let bytes = std::fs::read(path)
                .map_err(|e| anyhow::anyhow!("Failed to read wallpaper file: {}", e))?;
            let image = image::load_from_memory(&bytes)
                .map_err(|e| anyhow::anyhow!("Failed to load image: {}", e))?;
            let rgba = image.to_rgba8();
            let (width, height) = rgba.dimensions();
            let pixels = rgba.into_raw();

            Ok(iced::widget::image::Handle::from_rgba(width, height, pixels))
        })
        .await??;

        log::info!("Wallpaper loaded successfully");
        Ok(handle)
    }
}