use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Intent {
    pub target: String, // Кому (или "system")
    pub action: String, // Что сделать
    pub data: Vec<u8>,  // Параметры в бинарном виде
}

pub struct MessageBus {
    // В будущем тут будет очередь сообщений и регистратор подписчиков
}

impl MessageBus {
    pub async fn send_intent(intent: Intent) {
        println!("Bus: Processing intent {} for {}", intent.action, intent.target);
        // Логика маршрутизации ИИ или прямого вызова
    }
}