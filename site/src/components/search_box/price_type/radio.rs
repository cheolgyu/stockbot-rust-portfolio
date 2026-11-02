use std::str::FromStr;

use batch_model::app_type::price_type::PriceType;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use super::ElPriceTypes;

#[derive(Debug)]
pub struct RadioPriceType {}

#[derive(Clone, PartialEq, Properties, Debug)]
pub struct MyProperties {
    #[prop_or_default]
    pub checked: PriceType,
    #[prop_or_default]
    pub list: ElPriceTypes,
    #[prop_or_default]
    pub cb: Callback<PriceType>,
}
pub enum Msg {
    EventChange(InputEvent),
    // Init,
}

impl Component for RadioPriceType {
    type Message = Msg;
    type Properties = MyProperties;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::EventChange(e) => {
                let el = e.target_dyn_into::<HtmlInputElement>().unwrap();

                let el_attr_key = el.get_attribute("attr_key").unwrap();
                let key = PriceType::from_str(&el_attr_key).unwrap();
                ctx.props().cb.emit(key);
            }
        }
        true
    }

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let name = "price_type_radio";
        let on_event = ctx.link().callback(self::Msg::EventChange);
        let item = ctx.props().list.0.iter().map(|c| {
            let item = c.clone();
            let checked = ctx.props().checked== item.key;

            html!{
                <div>
                    <label for={item.key.to_string()} class="m-2 cursor-pointer">
                        <input class="checkbox_price_type checked:bg-blue-500 "  type="radio" {name} ID={item.key.to_string()} oninput={on_event.clone()} checked={checked} attr_key={item.key.to_string()} />
                        {item.text.to_string()}
                    </label>
                </div>
            }
        }).collect::<Html>();

        html! {
            <fieldset >
                <legend>{"가격종류"}</legend>
                {item}
            </fieldset>

        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {}

    fn prepare_state(&self) -> Option<String> {
        None
    }

    fn destroy(&mut self, _ctx: &Context<Self>) {}
}
