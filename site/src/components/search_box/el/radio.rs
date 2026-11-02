
use gloo::console::info;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use super::el_list::ElList;

#[derive(Debug)]
pub struct Radio;

#[derive(Clone, PartialEq, Properties, Debug)]
pub struct Properties {
    #[prop_or_default]
    pub data: ElList,
    pub cb: Callback<String>,
}
pub enum Msg {
    EventChange(InputEvent),
    // Init,
}

impl Component for Radio {
    type Message = Msg;
    type Properties = Properties;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::EventChange(e) => {
                let el = e.target_dyn_into::<HtmlInputElement>().unwrap();
                let el_attr_key = el.value();
                info!(&el_attr_key);
                ctx.props().cb.emit(el_attr_key);
            }
        }
        true
    }

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let item = get_random_buf().unwrap();
        let name = format!("{:?}", item);
        let on_event = ctx.link().callback(self::Msg::EventChange);
        let item = ctx.props().data.data.iter().map(|c| {
            let item = c.clone();

            html!{
                <>
                    <label for={item.key.to_string()} class="cursor-pointer">
                        <input class="checked:bg-blue-500 m-4 "  type="radio" name={name.clone()} id={item.key.clone()} 
                        oninput={on_event.clone()} checked={item.checked} value={item.key.to_string()} />
                        {item.text.to_string()}
                    </label>
                </>
            }
        }).collect::<Html>();

        html! {
            <>
                {item}
            </>

        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {}

    fn prepare_state(&self) -> Option<String> {
        None
    }

    fn destroy(&mut self, _ctx: &Context<Self>) {}
}

fn get_random_buf() -> Result<[u8; 32], getrandom::Error> {
    let mut buf = [0u8; 32];
    getrandom::getrandom(&mut buf)?;
    Ok(buf)
}
