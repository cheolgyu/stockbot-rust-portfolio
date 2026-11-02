use serde_json::Value;
use yew::{html, Html};

#[derive(Debug, Clone, PartialEq)]

pub struct TheadInfo<'a> {
    pub key: &'a str,
    pub label: &'a str,
    pub value_type: ColumnKind,
}

pub struct ColumnData<'a> {
    pub key: &'a str,
    pub value: &'a str,
}
#[derive(Debug, Clone, PartialEq)]
pub enum ColumnKind {
    PRICE,
    TEXT,
    NUMBER,
    NONE,
}
pub fn to_class(ck: &ColumnKind) -> &str {
    match ck {
        ColumnKind::PRICE => "ck_price",
        ColumnKind::TEXT => "ck_text",
        ColumnKind::NUMBER => "ck_number",
        ColumnKind::NONE => "",
    }
}

pub(crate) const TH_BOUND_CODE_DETAIL: [TheadInfo; 10] = [
    TheadInfo {
        key: "p1x",
        label: "시작일자",
        value_type: ColumnKind::TEXT,
    },
    TheadInfo {
        key: "p1y",
        label: "시작가격",
        value_type: ColumnKind::PRICE,
    },
    TheadInfo {
        key: "p2x",
        label: "종료일자",
        value_type: ColumnKind::TEXT,
    },
    TheadInfo {
        key: "p2y",
        label: "종료가격",
        value_type: ColumnKind::PRICE,
    },
    TheadInfo {
        key: "direction",
        label: "방향",
        value_type: ColumnKind::TEXT,
    },
    TheadInfo {
        key: "x_cnt",
        label: "변화 일수",
        value_type: ColumnKind::NUMBER,
    },
    TheadInfo {
        key: "y_cnt",
        label: "변화 가격",
        value_type: ColumnKind::PRICE,
    },
    TheadInfo {
        key: "ymxb_p3y",
        label: "직선 가격",
        value_type: ColumnKind::PRICE,
    },
    TheadInfo {
        key: "ymxb_m",
        label: "직선 기울기",
        value_type: ColumnKind::NUMBER,
    },
    TheadInfo {
        key: "ymxb_b",
        label: "직선 y절편",
        value_type: ColumnKind::NUMBER,
    },
];

pub(crate) const TH_BOUND: [TheadInfo; 11] = [
    TheadInfo {
        key: "p1x",
        label: "시작일",
        value_type: ColumnKind::TEXT,
    },
    TheadInfo {
        key: "p1y",
        label: "시작가격",
        value_type: ColumnKind::PRICE,
    },
    TheadInfo {
        key: "p2x",
        label: "종료일",
        value_type: ColumnKind::TEXT,
    },
    TheadInfo {
        key: "p2y",
        label: "종료가격",
        value_type: ColumnKind::PRICE,
    },
    TheadInfo {
        key: "direction",
        label: "방향",
        value_type: ColumnKind::TEXT,
    },
    TheadInfo {
        key: "x_cnt",
        label: "변화 일수",
        value_type: ColumnKind::NUMBER,
    },
    TheadInfo {
        key: "y_cnt",
        label: "변화 가격",
        value_type: ColumnKind::PRICE,
    },
    TheadInfo {
        key: "change_rate",
        label: "y백분율",
        value_type: ColumnKind::NUMBER,
    },
    TheadInfo {
        key: "ymxb_p3y",
        label: "직선 다음 가격",
        value_type: ColumnKind::PRICE,
    },
    TheadInfo {
        key: "ymxb_m",
        label: "직선 기울기",
        value_type: ColumnKind::NUMBER,
    },
    TheadInfo {
        key: "ymxb_b",
        label: "직선 y절편",
        value_type: ColumnKind::NUMBER,
    },
];

pub fn td_cell(column_kind: &ColumnKind, v: &Value) -> Html {
    let mut fv = format!("{}", v);
    if v.is_string() {
        let text = v.as_str().unwrap();
        fv = format!(" {} ", text);
    }
    let class_name = to_class(column_kind);

    html! {
        <td class={class_name.to_string()}> { fv }</td>
    }
}

pub(crate) const TH_VOLUME_CODE_DETAIL: [TheadInfo; 9] = [
    TheadInfo {
        key: "obs_type",
        label: "관측종류",
        value_type: ColumnKind::TEXT,
    },
    TheadInfo {
        key: "obs_val",
        label: "관측값",
        value_type: ColumnKind::NUMBER,
    },
    TheadInfo {
        key: "data_cnt",
        label: "관측 데이터 수",
        value_type: ColumnKind::NUMBER,
    },
    TheadInfo {
        key: "average",
        label: "데이터-평균",
        value_type: ColumnKind::NUMBER,
    },
    TheadInfo {
        key: "variance",
        label: "데이터-분산",
        value_type: ColumnKind::NUMBER,
    },
    TheadInfo {
        key: "std_dev",
        label: "데이터-표준편차",
        value_type: ColumnKind::NUMBER,
    },
    TheadInfo {
        key: "freq_val",
        label: "데이터-빈도값",
        value_type: ColumnKind::NUMBER,
    },
    TheadInfo {
        key: "percent",
        label: "데이터-퍼센트",
        value_type: ColumnKind::NUMBER,
    },
    TheadInfo {
        key: "data",
        label: "관측 데이터",
        value_type: ColumnKind::TEXT,
    },
];
pub(crate) const TH_VIEW_AGG_FREQ: [TheadInfo; 9] = [
    TheadInfo {
        key: "code",
        label: "코드",
        value_type: ColumnKind::TEXT,
    },
    TheadInfo {
        key: "name",
        label: "이름",
        value_type: ColumnKind::TEXT,
    },
    TheadInfo {
        key: "obs_epoch",
        label: "기간",
        value_type: ColumnKind::NUMBER,
    },
    TheadInfo {
        key: "obs_target",
        label: "최소, 최대, 평균",
        value_type: ColumnKind::NUMBER,
    },
    TheadInfo {
        key: "v_epoch_cnt",
        label: "관측대상의 관측 수",
        value_type: ColumnKind::NUMBER,
    },
    TheadInfo {
        key: "sum_v_epoch_cnt",
        label: "전체대상의 관측 수",
        value_type: ColumnKind::NUMBER,
    },
    TheadInfo {
        key: "rate_all_epoch",
        label: "전체에서 비중",
        value_type: ColumnKind::NUMBER,
    },
    TheadInfo {
        key: "rate_epoch",
        label: "비중-관측대상 중",
        value_type: ColumnKind::NUMBER,
    },
    TheadInfo {
        key: "v_obs_agg",
        label: "관측값",
        value_type: ColumnKind::TEXT,
    },
];
pub(crate) const TH_VIEW_AGG_BOUND_DETAIL_FREQ: [TheadInfo; 10] = [
    TheadInfo {
        key: "code",
        label: "코드",
        value_type: ColumnKind::TEXT,
    },
    TheadInfo {
        key: "name",
        label: "이름",
        value_type: ColumnKind::TEXT,
    },
    TheadInfo {
        key: "obs_epoch",
        label: "기간",
        value_type: ColumnKind::NUMBER,
    },
    TheadInfo {
        key: "obs_target",
        label: "최소, 최대, 평균",
        value_type: ColumnKind::NUMBER,
    },
    TheadInfo {
        key: "agg_bound_type",
        label: "바운드 종류",
        value_type: ColumnKind::TEXT,
    },
    TheadInfo {
        key: "v_epoch_cnt",
        label: "관측대상의 관측 수",
        value_type: ColumnKind::NUMBER,
    },
    TheadInfo {
        key: "sum_v_epoch_cnt",
        label: "전체대상의 관측 수",
        value_type: ColumnKind::NUMBER,
    },
    TheadInfo {
        key: "rate_all_epoch",
        label: "전체에서 비중",
        value_type: ColumnKind::NUMBER,
    },
    TheadInfo {
        key: "rate_epoch",
        label: "비중-관측대상 중",
        value_type: ColumnKind::NUMBER,
    },
    TheadInfo {
        key: "v_obs_agg",
        label: "관측값",
        value_type: ColumnKind::TEXT,
    },
];

pub(crate) const TH_VIEW_PRICE_CHG: [TheadInfo; 6] = [
    TheadInfo {
        key: "code",
        label: "코드",
        value_type: ColumnKind::TEXT,
    },
    TheadInfo {
        key: "name",
        label: "이름",
        value_type: ColumnKind::TEXT,
    },
    TheadInfo {
        key: "dt",
        label: "일자",
        value_type: ColumnKind::NUMBER,
    },
    TheadInfo {
        key: "lh",
        label: "저가 고가 비율",
        value_type: ColumnKind::NUMBER,
    },
    TheadInfo {
        key: "oc",
        label: "시가-종가 비율",
        value_type: ColumnKind::NUMBER,
    },
    TheadInfo {
        key: "vol",
        label: "전일 거래량 비율",
        value_type: ColumnKind::NUMBER,
    },
];

pub(crate) const TH_VIEW_PRICE_CHG_DETAIL: [TheadInfo; 4] = [
    TheadInfo {
        key: "dt",
        label: "일자",
        value_type: ColumnKind::NUMBER,
    },
    TheadInfo {
        key: "lh",
        label: "저가 고가 비율",
        value_type: ColumnKind::NUMBER,
    },
    TheadInfo {
        key: "oc",
        label: "시가-종가 비율",
        value_type: ColumnKind::TEXT,
    },
    TheadInfo {
        key: "vol",
        label: "전일 거래량 비율",
        value_type: ColumnKind::NUMBER,
    },
];
