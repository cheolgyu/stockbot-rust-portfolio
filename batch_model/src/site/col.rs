use std::default;

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Deserialize, Default)]
pub enum Fmt {
    Number,
    #[default]
    Text,
}
#[derive(Debug, PartialEq, Clone, Deserialize, Default)]
pub enum Align {
    Left,
    Right,
    #[default]
    Center,
}

#[derive(Debug, PartialEq, Clone, Deserialize, Default)]
pub struct Td {
    pub k: String,
    pub text: String,
    pub fmt: Fmt,
    pub align: Align,
    pub add_class: String,
}

#[derive(Debug, PartialEq, Clone, Deserialize, Default)]
pub struct Th {
    pub k: String,
    pub text: String,
    pub align: Align,
    pub add_class: String,
    pub clickable: bool,
}

pub trait ToTableV1 {
    fn th() -> Vec<Th>;
    fn td() -> Vec<Td>;
}

pub trait ToTableV2 {
    fn col() -> Vec<String>;
}
