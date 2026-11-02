use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Debug)]
pub struct InputObsVal {}
#[derive(Clone, PartialEq, Properties)]
pub struct Properties {
    pub cb: Callback<i32>,
}
pub enum Msg {
    EventChange(InputEvent),
}
impl Component for InputObsVal {
    type Message = Msg;
    type Properties = Properties;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::EventChange(e) => {
                let html_el = e.target_dyn_into::<HtmlInputElement>().unwrap();
                let v: i32 = html_el.value().parse().unwrap();
                ctx.props().cb.emit(v);
            }
        }
        true
    }

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_event = ctx.link().callback(self::Msg::EventChange);

        html! {
            <fieldset class="">
                <legend>{"관측값 "}</legend>
                <label >
                    <input class="focus:border-sky-500 focus:ring-1 focus:ring-sky-500" type="number" max="52" oninput={on_event.clone()}  />
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
