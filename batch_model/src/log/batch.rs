use chrono::{DateTime, Local};

use serde::{ser::SerializeTupleStruct, Deserialize, Serialize, Serializer};
use tb_row::TbRow;

#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[derive(Debug, PartialEq, Clone, Deserialize, Default, Serialize)]
pub struct LogBatch {
    //#[order(0)]
    pub batch_id: i64,
    //#[order(1)]
    pub batch_start_time: DateTime<Local>,
    //#[order(2)]
    pub batch_end_time: Option<DateTime<Local>>,
    //#[order(3)]
    pub batch_run_time_sec: Option<String>,

    //#[order(4)]
    pub step_id: i64,
    //#[order(5)]
    pub step_job_name: String,
    //#[order(6)]
    pub step_name: Option<String>,
    //#[order(7)]
    pub step_start_time: Option<DateTime<Local>>,
    //#[order(8)]
    pub step_end_time: Option<DateTime<Local>>,
    //#[order(9)]
    pub step_run_time_sec: Option<String>,
}
