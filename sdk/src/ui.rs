use crate::abi;

pub struct Button {
    label: String,
    x: i32,
    y: i32,
}

impl Button {
    pub fn new(label: &str) -> Self {
        Self { label: label.to_string(), x: 0, y: 0 }
    }

    pub fn position(mut self, x: i32, y: i32) -> Self {
        self.x = x; self.y = y;
        self
    }

    pub fn build(self) {
        unsafe {
            abi::ui_create_button(self.x, self.y, self.label.as_ptr(), self.label.len());
        }
    }
}