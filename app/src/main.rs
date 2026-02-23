use eframe::egui;
use wasmtime::{
    component::{bindgen, Component, Linker, ResourceTable},
    Config, Engine, Store,
};
use std::sync::{Arc, Mutex};

bindgen!({
    path: "../nexus.wit",
    world: "app",
});

// 1. Описываем, как кнопка выглядит в памяти Хоста
struct Button {
    label: String,
    x: i32,
    y: i32,
}

// 2. Общее состояние, доступное и WASM, и UI-движку
struct SharedState {
    buttons: Vec<Button>,
}

struct MyState {
    table: ResourceTable,
    shared: Arc<Mutex<SharedState>>, // Используем Arc/Mutex для потокобезопасности
}

// 3. Реализуем WIT-интерфейс
impl nexus::platform::ui::Host for MyState {
    fn create_button(&mut self, x: i32, y: i32, label: String) {
        let mut shared = self.shared.lock().unwrap();
        shared.buttons.push(Button { label, x, y });
        println!("[Nexus] WASM запросил кнопку: {}", x);
    }
}

fn main() -> eframe::Result {
    // Настраиваем состояние
    let shared_state = Arc::new(Mutex::new(SharedState { buttons: Vec::new() }));

    // Запускаем WASM в отдельном потоке (или до старта UI), чтобы он наполнил интерфейс
    let shared_for_wasm = shared_state.clone();
    std::thread::spawn(move || {
        if let Err(e) = run_wasm(shared_for_wasm) {
            eprintln!("WASM Error: {}", e);
        }
    });

    // Запускаем нативное окно
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Nexus Platform",
        options,
        Box::new(|_cc| Ok(Box::new(NexusApp { shared: shared_state }))),
    )
}

struct NexusApp {
    shared: Arc<Mutex<SharedState>>,
}

impl eframe::App for NexusApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Nexus Desktop Platform v0.1");
            
            let shared = self.shared.lock().unwrap();
            
            // РИСУЕМ ТО, ЧТО ПРИШЛО ИЗ WASM
            for btn in &shared.buttons {
                // В egui кнопки обычно в layout, но мы можем использовать fixed позиции
                let pos = egui::pos2(btn.x as f32, btn.y as f32);
                ui.put(egui::Rect::from_min_size(pos, egui::vec2(100.0, 30.0)), 
                    egui::Button::new(&btn.label)
                );
            }
        });
        
        // Постоянно перерисовываем, если WASM может добавить что-то новое
        ctx.request_repaint();
    }
}

// Функция инициализации WASM (почти не изменилась)
fn run_wasm(shared: Arc<Mutex<SharedState>>) -> anyhow::Result<()> {
    let mut config = Config::new();
    config.wasm_component_model(true);
    let engine = Engine::new(&config)?;
    let mut linker = Linker::new(&engine);
    nexus::platform::ui::add_to_linker(&mut linker, |s: &mut MyState| s)?;

    let mut store = Store::new(&engine, MyState { 
        table: ResourceTable::new(),
        shared 
    });

    let component_path = "target/wasm32-unknown-unknown/release/test_plugin.component.wasm";
    let component = Component::from_file(&engine, component_path)?;
    let app_world = App::instantiate(&mut store, &component, &linker)?;
    
    app_world.call_init(&mut store)?;
    Ok(())
}