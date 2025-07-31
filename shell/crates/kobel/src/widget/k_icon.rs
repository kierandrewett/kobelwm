use std::{path::PathBuf, sync::Arc};

use iced::widget::{row, svg};

use crate::{state::KobelShellState, KobelRootMessage};

pub fn k_icon<'a>(
    state: &'a Arc<KobelShellState>,
    name: impl Into<String>
) -> iced::Element<'a, KobelRootMessage, iced::Theme, iced::Renderer>
{
    let name_str: String = name.into();

    let name_pathbuf = PathBuf::from(name_str.clone());
    let icon_path = if name_pathbuf.has_root() {
        name_pathbuf
    } else {
        state.get_resource_path(&name_str)
    };

    if !icon_path.exists() || !icon_path.is_file() {
        log::warn!(
            "Failed to load icon. The path '{}' either does not exist or is not a valid file.",
            icon_path.display()
        );
    }

    match icon_path.extension() {
        Some(ext) if ext == "svg" => {
            return svg(&icon_path)
                .width(iced::Length::Fixed(16.0))
                .height(iced::Length::Fixed(16.0))
                .content_fit(iced::ContentFit::Contain)
                .symbolic(true)
                .style(move |_, _| svg::Style {
                    color: Some(state.shell_text_color),
                    ..Default::default()
                })
                .into();
        }
        Some(ext) if ext == "png" => {
            return iced::widget::image(&icon_path)
                .width(iced::Length::Fixed(16.0))
                .height(iced::Length::Fixed(16.0))
                .content_fit(iced::ContentFit::Contain)
                .into();
        }
        _ => {
            log::warn!("Unsupported icon format for '{}'. Only SVG and PNG are supported.", icon_path.display());
            return row![].into();
        }
    }
}