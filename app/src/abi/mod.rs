mod ui;
// mod network; // Будущий модуль

use wasmtime::*;
use crate::state::AppState;

pub fn register_all_abi(linker: &mut Linker<AppState>) -> Result<()> {
    // Вызываем регистраторы всех подсистем
    ui::register_ui(linker)?;
    // network::register_network(linker)?; 
    
    Ok(())
}