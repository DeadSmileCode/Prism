use wasmtime::*;
use crate::state::AppState;

pub fn register_ui(linker: &mut Linker<AppState>) -> Result<()> {
    // Регистрация кнопки
    linker.func_wrap("nexus", "ui_create_button", |mut caller: Caller<'_, AppState>, x: i32, y: i32, ptr: i32, len: i32| {
        let mem = caller.get_export("memory").unwrap().into_memory().unwrap();
        let data = mem.data(&caller);
        let label = std::str::from_utf8(&data[ptr as usize..(ptr + len) as usize]).unwrap();
        
        println!("[UI] Рисую кнопку: '{}' на {}, {}", label, x, y);
        caller.data_mut().counter += 1;
    })?;

    // Сюда можно добавлять ui_create_text, ui_create_slider и т.д.
    Ok(())
}