use std::{fmt, str::FromStr};

use serde::{Deserialize, Serialize};

pub mod parameter;
pub mod view_bound;
pub mod view_volume;

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug, Default)]
pub enum PageType {
    #[default]
    ViewBoardList,
    ViewVolumeList,
    List,
}
