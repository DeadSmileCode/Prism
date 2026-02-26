use iced::widget::{button, column, container, text};
use iced::{Color, Element, Length, Theme, Renderer};
use crate::ui::Message; // Импортируем Message из родительского модуля
use crate::state::AppState;


pub fn view(state: &AppState) -> Element<'_, Message, Theme, Renderer> {
    let sidebar_content = column(
        state.plugins
            .iter()
            .map(|p| {
                button(text(&p.name))
                    .width(Length::Fill)
                    .on_press(Message::PluginSelected(p.id.clone()))
                    .into()
            })
            .collect::<Vec<Element<Message, Theme, Renderer>>>()
    )
    .spacing(10)
    .padding(10);

    container(sidebar_content)
        .width(200)
        .height(Length::Fill)
        .style(|_theme| container::Style {
            background: Some(Color::from_rgb8(150, 150, 150).into()),
            ..Default::default()
        })
        .into()
}