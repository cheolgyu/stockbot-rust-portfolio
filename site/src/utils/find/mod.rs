use crate::components::search_box::el::{el_list::ElList, El};

pub fn el_what() -> ElList {
    ElList {
        data: vec![
            El {
                key: "H".to_string(),
                text: "머리".to_string(),
                checked: true,
            },
            El {
                key: "S".to_string(),
                text: "어깨".to_string(),
                checked: true,
            },
            El {
                key: "K".to_string(),
                text: "무릎".to_string(),
                checked: true,
            },
            El {
                key: "F".to_string(),
                text: "발".to_string(),
                checked: true,
            },
        ],
    }
}

pub fn el_why() -> ElList {
    ElList {
        data: vec![
            El {
                key: "B00".to_string(),
                text: "바운드".to_string(),
                checked: true,
            },
            El {
                key: "B10".to_string(),
                text: "오늘도 같은것".to_string(),
                checked: true,
            },
            El {
                key: "B11".to_string(),
                text: "오늘은 높은것".to_string(),
                checked: true,
            },
            El {
                key: "B12".to_string(),
                text: "오늘은 낮은것".to_string(),
                checked: true,
            },
            El {
                key: "V00".to_string(),
                text: "거래량".to_string(),
                checked: true,
            },
        ],
    }
}
