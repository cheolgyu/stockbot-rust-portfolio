pub use tb_row_derive::*;

pub trait TbRow {
    fn th_name() -> Vec<String>;
}
