use std::{path::PathBuf, sync::Arc};

use iced::core::image::FilterMethod;
use iced::widget::{container, row, svg};
use iced::Element;

use crate::util::debug::debug_border_style_or_default;
use crate::{state::KobelShellState, KobelRootMessage};

#[derive(Debug)]
pub struct KobelShellIcon<'a> {
    state: &'a Arc<KobelShellState>,
    name: String,

    size: iced::Length,
    is_symbolic: bool,
    color: Option<iced::Color>,
}

impl<'a> From<KobelShellIcon<'a>> for Element<'a, KobelRootMessage, iced::Theme, iced::Renderer> {
    fn from(icon: KobelShellIcon<'a>) -> Self {
        icon.view()
    }
}

impl<'a> KobelShellIcon<'a> {
    pub fn new(state: &'a Arc<KobelShellState>, name: impl Into<String>) -> Self {
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

        Self {
            state,
            name: icon_path.to_string_lossy().to_string(),
            size: iced::Length::Fixed(state.icon_base_size),
            is_symbolic: true,
            color: None,
        }
    }

    pub fn size(mut self, size: iced::Length) -> Self {
        self.size = size;
        self
    }

    pub fn symbolic(mut self, symbolic: bool) -> Self {
        self.is_symbolic = symbolic;
        self
    }

    pub fn color(mut self, color: Option<iced::Color>) -> Self {
        self.color = color;
        self
    }

    pub fn view(self) -> Element<'a, KobelRootMessage, iced::Theme, iced::Renderer> {
        let icon_path = PathBuf::from(self.name.clone());

        let icon_element: Element<'a, KobelRootMessage, iced::Theme, iced::Renderer> = match icon_path.extension() {
            Some(ext) if ext == "svg" => {
                svg(&icon_path)
                    .width(self.size)
                    .height(self.size)
                    .content_fit(iced::ContentFit::Contain)
                    .symbolic(self.is_symbolic)
                    .style(move |_, _| svg::Style {
                        color: if self.is_symbolic {
                            Some(self.color.unwrap_or(self.state.shell_text_color))
                        } else {
                            None
                        },
                        ..Default::default()
                    })
                    .into()
            }
            Some(ext) if ext == "png" => {
                iced::widget::image(&icon_path)
                    .width(self.size)
                    .height(self.size)
                    .content_fit(iced::ContentFit::Contain)
                    .filter_method(FilterMethod::Linear)
                    .into()
            }
            _ => {
                log::warn!("Unsupported icon format for '{}'. Only SVG and PNG are supported.", icon_path.display());
                return row![].into();
            }
        };

        container(icon_element)
            .style(move |theme: &iced::Theme| {
                container::Style {
                    border: debug_border_style_or_default(&self.state, iced::Border::default()),
                    ..Default::default()
                }
            })
            .into()
    }
}

pub fn k_icon<'a>(
    state: &'a Arc<KobelShellState>,
    name: impl Into<String>
) -> KobelShellIcon<'a>
{
    KobelShellIcon::new(state, name)
}
