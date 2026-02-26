use iced::widget::{button, column, container, row, text};
use iced::{Color, Element, Length, Task, Theme, Renderer};

use super::Message;

pub struct PrismWindow {
    title: String,
    scale_input: String,
    current_scale: f32,
    theme: Theme,
}

impl PrismWindow {
    pub fn new(count: usize) -> Self {
        Self {
            title: format!("Window_{count}"),
            scale_input: "1.0".to_string(),
            current_scale: 1.0,
            theme: Theme::ALL[count % Theme::ALL.len()].clone(),
        }
    }

    pub fn view(&self, id: iced::window::Id) -> Element<'_, Message> {
        iced::widget::button("+")
        .on_press(Message::OpenWindow)
        .into()
    }
}