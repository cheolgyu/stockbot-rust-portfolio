pub mod checkbox;
pub mod el_list;
pub mod radio;
pub mod select;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct El {
    pub key: String,
    pub text: String,
    pub checked: bool,
}
impl El {
    pub fn toggle(&mut self) {
        self.checked = !self.checked;
    }
}
