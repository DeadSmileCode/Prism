pub use nexus_sdk_macros::main;

#[macro_export]
macro_rules! generate_bindings {
    () => {
        wit_bindgen::generate!({
            path: "../../nexus.wit", 
            world: "app",
        });
        
        // Делаем модуль ui публичным внутри __nexus_internal
        pub use nexus::platform::ui;
    };
}