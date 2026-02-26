use std::collections::BTreeMap;

pub struct UIState {
    windows: BTreeMap<window::Id, Window>,
}