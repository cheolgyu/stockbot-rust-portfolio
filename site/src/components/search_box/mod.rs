pub mod el;
pub mod el_select;
pub mod exchange;
pub mod find;
pub mod input_date;
pub mod input_obs_val;
pub mod price_type;
pub mod search;

pub const OBS_TYPE_OPTIONS: [(&str, &str); 6] = [
    ("WeekMin", "주-최소"),
    ("WeekMax", "주-최대"),
    ("MonthMin", "월-최소"),
    ("MonthMax", "월-최대"),
    ("QuarterMin", "분기-최소"),
    ("QuarterMax", "분기-최대"),
];

pub const OBS_TARGET_OPTIONS: [(&str, &str); 3] =
    [("MIN", "최소"), ("MAX", "최대"), ("AVG", "평균")];
pub const AGG_BOUND_TYPE_OPTIONS: [(&str, &str); 2] =
    [("CHG", "가격 변화율"), ("DAY", "바운드 지속일")];
pub const OBS_EPOCH_OPTIONS: [(&str, &str); 3] =
    [("WEEK", "주"), ("MONTH", "월"), ("QUARTER", "분기")];
