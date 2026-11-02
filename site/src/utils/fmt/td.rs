use serde_json::Value;
use yew::{classes, html, Html};

pub fn td(price_class_nm: &str, v: &Value) -> Html {
    let mut fv = format!("{}", v);
    if v.is_string() {
        let text = v.as_str().unwrap();
        fv = format!(" {} ", text);
    }
    let nm = price_class_nm.to_string();

    html! {
        <td class={classes!(nm)}> { fv }</td>
    }
}
