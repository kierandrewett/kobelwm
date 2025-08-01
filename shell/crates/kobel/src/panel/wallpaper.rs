use std::sync::Arc;

use iced::{core::window, platform_specific::shell::commands::{layer_surface::get_layer_surface, subsurface::{Anchor, KeyboardInteractivity, Layer}}, widget::{container, mouse_area, row}, window::Id, Background, Color, Element, Rectangle, Task};
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
    wallpaper_fade_start_time: Option<std::time::Instant>,
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
            pointer_interactivity: true,
            ..Default::default()
        });

        (
            Self {
                id,
                state: state.clone(),

                wallpaper_opacity: 0.0,
                wallpaper_handle: None,
                wallpaper_fade_start_time: None,
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
        if let KobelRootMessage::Panel(crate::panel::KobelPanelMessage::Wallpaper(wallpaper_message)) = &message {
            match wallpaper_message {
                KobelWallpaperMessage::WallpaperLoaded(handle) => {
                    self.wallpaper_handle = Some(handle.clone());
                    self.wallpaper_fade_start_time = Some(std::time::Instant::now());

                    log::info!("Wallpaper loaded successfully");
                }
                KobelWallpaperMessage::WallpaperFailed => {
                    log::error!("Failed to load wallpaper");
                }
            }
        }

        match message {
            KobelRootMessage::Tick(_) => {
                if self.wallpaper_fade_start_time.is_some() {
                    let elapsed = self.wallpaper_fade_start_time.unwrap().elapsed().as_secs_f32();
                    let duration = 1.5;

                    if elapsed > duration {
                        self.wallpaper_opacity = 1.0;
                    } else {
                        let opacity = (elapsed / duration).clamp(0.0_f32, 1.0_f32);
                        let eased_opacity = easing_function::easings::Linear::ease(opacity);
    
                        self.wallpaper_opacity = eased_opacity;
                    }
                }
            }
            _ => {}
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

        mouse_area(
            container(wallpaper_element)
                .width(iced::Length::Fill)
                .height(iced::Length::Fill)
                .align_x(iced::Alignment::Center)
                .align_y(iced::Alignment::Center)
                .style(move |_| container::Style {
                    background: Some(Background::Color(Color::BLACK)),
                    ..container::Style::default()
                })
        )
            .on_right_press(KobelRootMessage::OpenContextMenu { width: 250.0, height: 187.0 })
            .into()
    }

    async fn load_wallpaper(state: Arc<KobelShellState>) -> anyhow::Result<iced::widget::image::Handle> {
        log::info!("Loading wallpaper...");
                
        let path = state.get_resource_path(&state.shell_wallpaper_name);
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