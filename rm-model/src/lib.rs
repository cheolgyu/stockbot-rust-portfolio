use serde::{Deserialize, Serialize};

// #[cfg_attr(not(target_arch = "wasm32"), derive(sqlx::FromRow))]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct Bound {
    pub code_id: i64,
}
