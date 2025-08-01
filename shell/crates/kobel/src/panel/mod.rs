pub mod bar;
pub mod dock;
pub mod wallpaper;
pub mod debug;
pub mod context_menu;
pub mod search;

#[derive(Debug, Clone)]
pub enum KobelPanelMessage {
    Bar(bar::KobelBarMessage),
    Dock(dock::KobelDockMessage),
    Wallpaper(wallpaper::KobelWallpaperMessage),
    Debug(debug::KobelDebugMessage),
    ContextMenu(context_menu::KobelContextMenuMessage),
    Search(search::KobelSearchMessage),
}