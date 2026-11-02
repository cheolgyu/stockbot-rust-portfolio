use batch_model::app_type::price_type::PriceType;

pub mod checkbox;
pub mod radio;

#[derive(Debug, Clone, PartialEq)]
pub struct ElPriceType {
    pub key: PriceType,
    pub text: String,
    pub checked: bool,
}

impl ElPriceType {
    pub fn toggle(&mut self) {
        self.checked = !self.checked;
    }
}

#[derive(Clone, PartialEq, Debug, Default)]
pub struct ElPriceTypes(pub Vec<ElPriceType>);

impl ElPriceTypes {
    pub fn udpate_toggle(&mut self, key: PriceType) {
        for v in self.0.iter_mut() {
            if v.key == key {
                v.toggle();
            }
        }
    }
    pub fn udpate_radio_select(&mut self, key: PriceType) {
        for v in self.0.iter_mut() {
            v.checked = v.key == key;
        }
    }
    pub fn convert_key(key: &str) -> Option<String> {
        match key {
            "cp" => Some("CLOSE".to_string()),
            "op" => Some("OPEN".to_string()),
            "lp" => Some("LOW".to_string()),
            "hp" => Some("HIGH".to_string()),
            _ => None,
        }
    }
    pub fn new() -> Self {
        Self(vec![
            ElPriceType {
                key: PriceType::CLOSE,
                text: "종가".to_string(),
                checked: true,
            },
            ElPriceType {
                key: PriceType::OPEN,
                text: "시가".to_string(),
                checked: false,
            },
            ElPriceType {
                key: PriceType::LOW,
                text: "저가".to_string(),
                checked: false,
            },
            ElPriceType {
                key: PriceType::HIGH,
                text: "고가".to_string(),
                checked: false,
            },
        ])
    }
}
