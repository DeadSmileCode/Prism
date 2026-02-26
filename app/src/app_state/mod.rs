pub struct PluginInfo {
    pub id: String,
    pub name: String,
    pub description: String,
}

pub struct AppState {
    pub plugins: Vec<PluginInfo>,
    pub active_plugin_id: Option<String>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            plugins: Vec::new(),
            active_plugin_id: None,
        }
    }
}