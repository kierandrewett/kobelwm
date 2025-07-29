use iced::core::Element;

pub mod button;

pub fn button<'a, Message, Theme, Renderer>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> button::Button<'a, Message, Theme, Renderer>
where
    Theme: button::Catalog + 'a,
    Renderer: iced::core::Renderer,
{
    button::Button::new(content)
}