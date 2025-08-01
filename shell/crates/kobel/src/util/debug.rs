use std::sync::Arc;

use iced::Color;

use crate::state::KobelShellState;

pub fn debug_border_style_or_default(state: &Arc<KobelShellState>, value: iced::Border) -> iced::Border {
    let enabled = *state.debug_border_style.read().unwrap_or_else(|_| panic!("Failed to acquire read lock"));

    if enabled {
        iced::Border {
            color: Color::from_rgb(1.0, 0.0, 0.0),
            width: 1.0,
            ..value
        }
    } else {
        value
    }
}