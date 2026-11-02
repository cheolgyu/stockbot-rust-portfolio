use std::error::Error;

use num_format::{Locale, ToFormattedString};
use serde_json::Value;
use yew::{classes, html, Html};

fn num_fmt(v: &Value) -> Result<String, Box<dyn Error>> {
    // info!("==========================");
    // info!("=========================={:?}", v.to_string());

    let s = if v.is_string() {
        let v1 = v.to_string();

        v.as_str()
            .unwrap()
            .parse::<i64>()
            .unwrap()
            .to_formatted_string(&Locale::en)
    } else if v.is_f64() {
        v.to_string()
    } else {
        let v2 = v.to_string();
        //info!("v2========================={:?}", v2.clone());

        v2.parse::<i64>().unwrap().to_formatted_string(&Locale::en)
    };
    Ok(s)
}

pub fn td_num(price_class_nm: &str, v: &Value) -> Html {
    let c = format!("td_num {}", price_class_nm);
    html! {
        <td class={classes!(c)}> { num_fmt(v).unwrap() }</td>
    }
}
