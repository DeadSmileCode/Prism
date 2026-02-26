pub mod ui;

use wasmtime::component::Linker;

pub fn add_to_linker<T>(linker: &mut Linker<T>) -> anyhow::Result<()> {
    // Обращаемся к функции из файла abi/ui.rs
    ui::register_ui(linker)?;
    Ok(())
}