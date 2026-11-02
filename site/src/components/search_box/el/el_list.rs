use serde::{Deserialize, Serialize};

use super::El;

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ElList {
    pub data: Vec<El>,
}
impl ElList {
    pub fn check_one(&mut self, key: &str, else_false: bool) {
        for i in self.data.iter_mut() {
            // info!(
            //     "i.key={}====key={}======>res ={}",
            //     &i.key,
            //     key.to_string(),
            //     i.key == key.to_string()
            // );
            if i.key == key {
                i.checked = true;
            } else if else_false {
                i.checked = false;
            }
        }
    }
}
