pub mod bar;
pub mod dock;
pub mod wallpaper;
pub mod debug;

#[derive(Debug, Clone)]
pub enum KobelPanelMessage {
    Bar(bar::KobelBarMessage),
    Dock(dock::KobelDockMessage),
    Wallpaper(wallpaper::KobelWallpaperMessage),
    Debug(debug::KobelDebugMessage),
}