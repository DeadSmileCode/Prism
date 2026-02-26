// use crate::app_state::AppState as AppState;
pub mod window;


use iced::{message};
use iced::widget::{
    button, center, center_x, column, container, operation, scrollable, space, text, text_input,
};
use iced::{Center, Element, Fill, Function, Subscription, Task, Theme, Vector};

use std::{collections::BTreeMap, fs::Metadata};

use crate::ui::window::PrismWindow;

// mod sidebar; 
// use iced::widget::{
//     button, center, center_x, column, container, operation, scrollable, space, text, text_input,
// };
// use iced::window;
// use iced::{Center, Element, Fill, Function, Subscription, Task, Theme, Vector};




pub struct PrismUI {
    windows: BTreeMap<iced::window::Id, PrismWindow>,
}


#[derive(Debug, Clone)]
pub enum Message {
    OpenWindow,
    WindowOpened(iced::window::Id),
    WindowClosed(iced::window::Id),
}



impl PrismUI {
    pub fn new() -> (Self, Task<Message>) {
        let (_, open) = iced::window::open(iced::window::Settings::default());

        (
            Self {
                windows:BTreeMap::new(),
            },
            open.map(Message::WindowOpened),
        )
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::OpenWindow => {
                let Some(last_window) = self.windows.keys().last() else {
                    return Task::none();
                };

                iced::window::position(*last_window)
                    .then(|last_position| {
                        let position =
                            last_position.map_or(iced::window::Position::Default, |last_position| {
                                iced::window::Position::Specific(last_position + Vector::new(20.0, 20.0))
                            });

                        let (_, open) = iced::window::open(iced::window::Settings {
                            position,
                            ..iced::window::Settings::default()
                        });

                        open
                    })
                    .map(Message::WindowOpened)

            }
            Message::WindowOpened(id) => {
                let window = PrismWindow::new(self.windows.len() + 1);
                let focus_input = operation::focus(format!("input-{id}"));

                self.windows.insert(id, window);

                focus_input
            }
            Message::WindowClosed(id) => {
                self.windows.remove(&id);

                if self.windows.is_empty() {
                    iced::exit()
                } else {
                    Task::none()
                }
            }
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        iced::window::close_events().map(Message::WindowClosed)
    }

    pub fn view(&self, window_id: iced::window::Id) -> Element<'_, Message> {
        if let Some(window) = self.windows.get(&window_id) {
            center(window.view(window_id)).into()
        } else {
            space().into()
        }
    }
    
}

// pub struct NexusShell {
//     pub state: AppState,
// }

// #[derive(Debug, Clone)]
// pub enum Message {
//     PluginSelected(String),
//     #[allow(dead_code)]
//     IntentReceived(String),
// }

// // 1. Экспортируем функцию для ABI, чтобы main её видел
// pub fn register_ui<T>(_linker: &mut wasmtime::component::Linker<T>) -> anyhow::Result<()> {
//     Ok(())
// }

// impl NexusShell {
//     // 2. Функция инициализации для iced::application
//     pub fn new() -> (Self, Task<Message>) {
//         (
//             Self {
//                 state: AppState::new(),
//             },
//             Task::none(),
//         )
//     }

//     // pub fn title(&self) -> String {
//     //     String::from("Nexus Desktop")
//     // }

//     pub fn update(&mut self, message: Message) -> Task<Message> {
//         match message {
//             Message::PluginSelected(id) => {
//                 self.state.active_plugin_id = Some(id);
//             }
//             _ => {}
//         }
//         Task::none()
//     }

//     pub fn view(&self) -> Element<'_, Message, Theme, Renderer> {
//         // 2. Вызываем функцию из модуля sidebar
//         let sidebar = sidebar::view(&self.state);

//         let content = container(
//             text(self.state.active_plugin_id.as_deref().unwrap_or("Выберите плагин"))
//         )
//         .width(Length::Fill)
//         .height(Length::Fill)
//         .center_x(Length::Fill)
//         .center_y(Length::Fill);

//         // 3. Собираем всё вместе
//         row![sidebar, content].into()
//     }
// }