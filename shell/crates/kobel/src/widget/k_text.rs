use std::sync::Arc;

use iced::{widget::{container, svg, text}, Element};

use crate::{state::KobelShellState, util::debug::debug_border_style_or_default, KobelRootMessage};

#[derive(Debug)]
pub struct KobelShellText<'a> {
    state: &'a Arc<KobelShellState>,
    contents: text::Fragment<'a>,

    size: f32,
    is_bold: bool,
}

impl<'a> From<KobelShellText<'a>> for Element<'a, KobelRootMessage, iced::Theme, iced::Renderer> {
    fn from(text: KobelShellText<'a>) -> Self {
        text.view()
    }
}

impl<'a> KobelShellText<'a> {
    pub fn new(state: &'a Arc<KobelShellState>, contents: impl text::IntoFragment<'a>) -> Self {
        Self {
            state,
            contents: contents.into_fragment(),
            size: 1.0,
            is_bold: false,
        }
    }

    pub fn bold(mut self, is_bold: bool) -> Self {
        self.is_bold = is_bold;
        self
    }

    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    pub fn view(self) -> Element<'a, KobelRootMessage, iced::Theme, iced::Renderer> {
        let font = if self.is_bold {
            self.state.font_bold
        } else {
            self.state.font
        };

        let font_size = self.size * self.state.font_base_size;

        container(
            text(self.contents.clone())
                .font(font)
                .size(font_size)
        )
            .style(move |_| container::Style {
                border: debug_border_style_or_default(&self.state, iced::Border::default()),
                ..Default::default()
            })
            .into()
    }
}

pub fn k_text<'a>(
    state: &'a Arc<KobelShellState>,
    contents: impl text::IntoFragment<'a>
) -> KobelShellText<'a>
{
    KobelShellText::new(state, contents)
}