use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlSelectElement};
use yew::prelude::*;

#[derive(Clone, PartialEq, Debug)]
pub struct ElOptions(pub Vec<(String, String)>);
#[derive(Clone)]
pub struct SelectOption {
    pub key: String,
    pub val: String,
    pub selected: bool,
}

#[derive(Clone, PartialEq, Properties, Debug)]
pub struct Properties {
    pub cb: Callback<String>,
    pub default: String,
    pub options: ElOptions,
    #[prop_or_default]
    pub legend: Option<String>,
}
pub enum Msg {
    EventChange(InputEvent),
}

pub struct ElSelect {}
impl Component for ElSelect {
    type Message = Msg;
    type Properties = Properties;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::EventChange(e) => {
                let target: Option<EventTarget> = e.target();
                let input: HtmlSelectElement = target
                    .and_then(|t| t.dyn_into::<HtmlSelectElement>().ok())
                    .unwrap();

                ctx.props().cb.emit(input.value());
            }
        }
        true
    }

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
           <>
                <fieldset class=""><legend>{ ctx.props().legend.clone() }</legend>
                    {self.view_select(ctx)}
                </fieldset>

           </>

        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {}

    fn prepare_state(&self) -> Option<String> {
        None
    }

    fn destroy(&mut self, _ctx: &Context<Self>) {}
}

impl ElSelect {
    pub fn view_select(&self, ctx: &Context<Self>) -> Html {
        let on_event = ctx.link().callback(self::Msg::EventChange);
        let options = ctx
            .props()
            .options
            .0
            .iter()
            .map(|(k, v)| {
                let k = k.clone();
                let item = SelectOption {
                    key: k.clone(),
                    val: v.clone(),
                    selected: k == ctx.props().default,
                };
                html! {
                    self.view_option(item)
                }
            })
            .collect::<Html>();

        html! {
            <>
            <select class="w-full" oninput={on_event}>
                {options}
            </select>
            </>
        }
    }

    pub fn view_option(&self, item: SelectOption) -> Html {
        html! {
            <option value={item.key} selected={item.selected}> {format!("{}", item.val)} </option>
        }
    }
}
