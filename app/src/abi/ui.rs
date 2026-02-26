use iced::widget::{button, column, container, row, text};
use iced::{Color, Element, Length, Task, Theme};
use crate::state::AppState;

pub struct NexusShell {
    pub state: AppState,
}

#[derive(Debug, Clone)]
pub enum Message {
    PluginSelected(String),
    #[allow(dead_code)]
    IntentReceived(String),
}

// Эта функция остается без изменений, так как она относится к логике WASM
pub fn register_ui<T>(_linker: &mut wasmtime::component::Linker<T>) -> anyhow::Result<()> {
    Ok(())
}

impl NexusShell {
    // 1. Вместо Sandbox::new() теперь функция, возвращающая состояние и начальную задачу
    pub fn new() -> (Self, Task<Message>) {
        (
            Self {
                state: AppState::new(),
            },
            Task::none(),
        )
    }

    pub fn title(&self) -> String {
        String::from("Nexus Desktop")
    }

    // 2. Update теперь возвращает Task<Message>
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::PluginSelected(id) => {
                self.state.active_plugin_id = Some(id);
            }
            Message::IntentReceived(_intent) => {
                // Обработка интентов
            }
        }
        Task::none()
    }

    // 3. View теперь возвращает Element<'_, Message>
    pub fn view(&self) -> Element<Message> {
        // Создаем кнопки для сайдбара
        let sidebar_content = column(
            self.state.plugins
                .iter()
                .map(|p| {
                    button(text(&p.name))
                        .width(Length::Fill)
                        .on_press(Message::PluginSelected(p.id.clone()))
                        .into()
                })
                .collect::<Vec<Element<Message>>>()
        )
        .spacing(10)
        .padding(10);

        // Чтобы покрасить сайдбар, оборачиваем колонку в контейнер
        let sidebar = container(sidebar_content)
            .width(200)
            .height(Length::Fill)
            .style(|_theme| container::Style {
                // Тёмно-серый фон для сайдбара
                background: Some(Color::from_rgb8(45, 45, 48).into()),
                ..Default::default()
            });

        // Основная рабочая область
        let active_text = self.state.active_plugin_id
            .as_deref()
            .unwrap_or("Выберите плагин");

        let content = container(
            text(active_text).size(20)
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill) // В 0.14 требует указания Fill для центровки
        .center_y(Length::Fill);

        row![sidebar, content].into()
    }
}