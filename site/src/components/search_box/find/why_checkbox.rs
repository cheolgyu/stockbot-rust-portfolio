// use std::str::FromStr;

// use gloo_utils::window;
// use site_common::model::compoent::exchange::{Key, CheckboxExchange, Checkbox};
// use wasm_bindgen::JsCast;
// use web_sys::HtmlInputElement;
// use yew::prelude::*;

// use crate::request;

// #[derive(Debug)]
// pub struct CheckBoxExchange {
//     base: Vec<Checkbox>,
// }
// #[derive(Clone, PartialEq, Properties)]
// pub struct Properties {
//     pub cb_exchange: Callback<Vec<String>>,
//     pub params: String,
// }
// pub enum Msg {
//     EventChange(InputEvent),
//     Fetch,
//     FetchOk(Vec<String>),
//     FetchErr,
// }

// impl Component for CheckBoxExchange {
//     type Message = Msg;
//     type Properties = Properties;

//     fn create(ctx: &Context<Self>) -> Self {
//         ctx.link().send_future(async { Msg::Fetch });
//         Self { base: vec![] }
//     }

//     fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
//         match msg {
//             Msg::Fetch => {
//                 ctx.link().send_future(async {
//                     match request::exchange::get_list().await {
//                         Ok(result) => Msg::FetchOk(result),
//                         Err(_) => Msg::FetchErr,
//                     }
//                 });
//             }
//             Msg::FetchOk(result) => {
//                 self.base = CheckboxExchange::new_checked(result).0;

//                 println!("exe Msg::FetchOk");
//             }
//             Msg::FetchErr => {
//                 println!("exe Msg::FetchErr");
//             }
//             Msg::EventChange(e) => {
//                 let el_attr_key = e
//                     .target_dyn_into::<HtmlInputElement>()
//                     .unwrap()
//                     .get_attribute("attr_key")
//                     .unwrap();
//                 for x in 0..self.base.len() {
//                     if self.base[x].key.to_string() == el_attr_key {
//                         self.base[x].checked = !self.base[x].checked;
//                     }
//                 }

//                 let mut res: Vec<String> = vec![];

//                 let html_coll = window()
//                     .document()
//                     .unwrap()
//                     .get_elements_by_class_name("checkbox_exchange");
//                 for i in 0..html_coll.length() {
//                     let el = html_coll.get_with_index(i).unwrap();

//                     let inp_el: HtmlInputElement = el.dyn_into::<HtmlInputElement>().unwrap();
//                     if inp_el.checked() {
//                         res.push(inp_el.get_attribute("attr_key").unwrap());
//                     }
//                 }

//                 ctx.props().cb_exchange.emit(res);
//             }
//         }
//         true
//     }

//     fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
//         true
//     }

//     fn view(&self, ctx: &Context<Self>) -> Html {
//         let on_event = ctx.link().callback(|e| self::Msg::EventChange(e));
//         let paramm_exist = ctx.props().params.clone();

//         let item = self.base.clone().iter().map(|c| {
//             let split = paramm_exist.split(",");
//             let item = c.clone();
//             let mut params_exist =false;

//             for vs in split  {
//                 if Key::from_str(vs).unwrap() == item.key{
//                     params_exist = true;
//                 }
//             }

//             html!{
//                 <div>
//                     <label for={item.key.to_string()}>
//                         <input class="checkbox_exchange checked:bg-blue-500"  type="checkbox" name={item.key.to_string()} id={item.key.to_string()} oninput={on_event.clone()} checked={params_exist} attr_key={item.key.to_string()} />
//                         {item.text}
//                     </label>
//                 </div>
//             }
//         }).collect::<Html>();

//         html! {
//             <fieldset class="">
//                 <legend>{"거래소"}</legend>
//                 {item}
//             </fieldset>

//         }
//     }

//     fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {}

//     fn prepare_state(&self) -> Option<String> {
//         None
//     }

//     fn destroy(&mut self, _ctx: &Context<Self>) {}
// }
