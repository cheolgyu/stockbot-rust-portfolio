use chrono::NaiveDate;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Debug)]
pub struct InputDate {}
#[derive(Clone, PartialEq, Properties)]
pub struct Properties {
    pub cb: Callback<NaiveDate>,
    pub default: Option<NaiveDate>,
}
pub enum Msg {
    EventChange(InputEvent),
}
const FORMAT_STR: &str = "%Y-%m-%d";

impl Component for InputDate {
    type Message = Msg;
    type Properties = Properties;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::EventChange(e) => {
                let html_el = e.target_dyn_into::<HtmlInputElement>().unwrap();
                let naive_date = NaiveDate::parse_from_str(&html_el.value(), FORMAT_STR)
                    .expect("Failed to parse date string");

                ctx.props().cb.emit(naive_date);
            }
        }
        true
    }

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_event = ctx.link().callback(self::Msg::EventChange);
        let val = match ctx.props().default {
            Some(v) => v.to_string(),
            None => "".to_string(),
        };

        html! {
            <fieldset class="">
                <legend>{"일자"}</legend>
                <label >
                    <input type="date" class="focus:border-sky-500 focus:ring-1 focus:ring-sky-500"  value={val}  oninput={on_event.clone()}  />
                </label>
            </fieldset>

        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {}

    fn prepare_state(&self) -> Option<String> {
        None
    }

    fn destroy(&mut self, _ctx: &Context<Self>) {}
}
