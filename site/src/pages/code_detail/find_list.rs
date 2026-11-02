use std::str::FromStr;

use crate::{
    components::{
        paging::Paging,
        search_box::el::{el_list::ElList, radio::Radio},
    },
    request,
    utils::{
        self,
        thead_info::{td_cell, ColumnKind},
    },
};
use batch_model::{
    app_type::{find_what::What, find_why::Why},
    find::view::by_code::FindViewByCode,
    site::{param::find_by_code::Param, res::PageResult},
};
use log::info;
use serde_json::{json, Map, Value};

use web_sys::HtmlElement;
use yew::prelude::*;
//https://docs.rs/yew-router/latest/src/yew_router/components/link.rs.html#65
//https://github.com/yewstack/yew/blob/master/examples/router/src/pages/post_list.rs

type Res = PageResult<FindViewByCode>;

pub enum Msg {
    //https://yew.rs/docs/concepts/suspense#render-as-you-fetch
    Fetch,
    FetchOk(Res),
    FetchErr,
    EventTh(MouseEvent),
    EventPage(usize),
    EventWhat(String),
    EventWhy(String),
}
#[derive(Clone, PartialEq, Properties)]
pub struct Properties {
    pub code: String,
    pub code_id: i64,
}
pub struct FindList {
    param: Param,
    data: Res,
    why_radio: ElList,
    what_radio: ElList,
}

impl Component for FindList {
    type Message = Msg;
    type Properties = Properties;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(Msg::Fetch);

        let mut i = Self {
            data: Res::default(),
            param: Param::default(),
            why_radio: utils::find::el_why(),
            what_radio: utils::find::el_what(),
        };

        let de = i.param.why.to_string();
        info!("{:?}", de);

        let code_id = ctx.props().code_id;
        i.param.code_id = code_id;

        i.what_radio.check_one(&i.param.what.to_string(), true);
        i.why_radio.check_one(&i.param.why.to_string(), true);
        info!("{:?}", &i.what_radio);
        info!("{:?}", &i.why_radio);
        i
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        //let mut param = &self.param;
        match msg {
            Msg::Fetch => {
                let param = self.param.clone();

                ctx.link().send_future(async {
                    match request::fetch(param).await {
                        Ok(page_result) => {
                            let res: Res = serde_json::from_value(page_result).unwrap();
                            Msg::FetchOk(res)
                        }
                        Err(_err) => Msg::FetchErr,
                    }
                });
            }
            Msg::FetchOk(page_result) => {
                self.data = page_result;
            }
            Msg::FetchErr => {
                info!("exe Msg::FetchErr");
            }
            Msg::EventTh(e) => {
                let t = e.target_dyn_into::<HtmlElement>().unwrap();
                let k = t.get_attribute("click_id").unwrap();
                self.param.paging.set_sort(k);
                ctx.link().send_message(Msg::Fetch);
            }
            Msg::EventPage(v) => {
                self.param.paging.page = v;
                ctx.link().send_message(Msg::Fetch);
            }
            Msg::EventWhat(v) => {
                self.what_radio.check_one(&v.to_string(), true);
                self.param.what = What::from_str(&v).unwrap();
                ctx.link().send_message(Msg::Fetch);
            }
            Msg::EventWhy(v) => {
                self.why_radio.check_one(&v.to_string(), true);
                self.param.why = Why::from_str(&v).unwrap();
                ctx.link().send_message(Msg::Fetch);
            }
        }
        true
    }

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link().clone();
        let cb_paging: Callback<usize> =
            Callback::from(move |v: usize| link.send_message(Msg::EventPage(v)));

        let tbody = self.tbody(ctx);
        let thead = self.thead(ctx).clone();

        let info = self.data.page_info.clone();
        let cnt = format!("( {} 건)", self.data.page_info.total_cnt);

        let jp: Map<String, Value> = json!(self.param).as_object().unwrap().clone();

        let what_radio_callback = ctx.link().callback(Msg::EventWhat);
        let what_radio = self.what_radio.clone();
        let why_radio_callback = ctx.link().callback(Msg::EventWhy);
        let why_radio = self.why_radio.clone();

        html! {
            <>
            <hr/>
            <h1>{ "파인트 목록..." }</h1>


            <div class="flex flex-row place-content-between ">
            </div>

            <div class="flex flex-row">

                <div class="p-2">
                    <fieldset >
                        <legend>{"출력: Why 종류"}</legend>
                        <Radio data={why_radio} cb={why_radio_callback} ></Radio>
                    </fieldset >
                </div>

                <div class="p-2">
                    <fieldset >
                        <legend>{"출력: What 종류"}</legend>
                        <Radio data={what_radio} cb={what_radio_callback} ></Radio>
                    </fieldset >
                </div>
            </div>

            <hr/>
            <div class="flex table-fixed  overflow-auto">
                <table  >
                    <caption>{"파인트 목록"} {cnt} </caption>
                    <colgroup>
                        <col/>
                        <col/>
                        <col/>
                        <col/>
                        <col/>

                        <col/>
                        <col/>
                    </colgroup>
                    {thead}
                    {tbody}
                </table>
            </div>

            <div class="flex flex-row ">
                <Paging
                    info={info}
                    cb={cb_paging}
                    params={jp}
                 ></Paging>
            </div>
            </>
        }
    }

    //https://yew.rs/docs/advanced-topics/struct-components/lifecycle#rendered
    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {}

    fn prepare_state(&self) -> Option<String> {
        None
    }

    fn destroy(&mut self, _ctx: &Context<Self>) {}
}

impl FindList {
    fn thead(&self, ctx: &Context<Self>) -> Html {
        let on_clicked = ctx.link().callback(self::Msg::EventTh);

        let th_set1 = get_th()
            .iter()
            .map(|(k, v)| {
                html! {
                    <th class="cursor-pointer" rowspan="3" click_id={*k} onclick={on_clicked.clone()} >{v}</th>
                }
            })
            .collect::<Html>();

        html! {
            <thead class="">
                <tr>
                    {th_set1}
                </tr>
            </thead>
        }
    }

    fn tbody(&self, _ctx: &Context<Self>) -> Html {
        let td_set1 = self
            .data
            .data
            .iter()
            .map(|item| {
                let why = &Value::String(item.why.text());
                let line = &Value::String(item.line.to_text().to_string());

                let j = json!(item);
                let v = j.as_object().unwrap();

                html! {
                    <tr>
                         {td_cell(&ColumnKind::TEXT,&v["dt"])}
                        {td_cell(&ColumnKind::TEXT,why)}
                        {td_cell(&ColumnKind::NUMBER,&v["order_num"])}
                        {td_cell(&ColumnKind::TEXT, line)}
                        {td("",&v["p1x"])}

                        {td_cell(&ColumnKind::NUMBER,&v["change_rate"])}
                    </tr>
                }
            })
            .collect::<Html>();
        html! {
            <tbody>
                {td_set1}
            </tbody>
        }
    }
}
fn td(price_class_nm: &str, v: &Value) -> Html {
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
// pub fn get_td(&self) -> Option<Vec<Value>> {
//     let mut res: Vec<Value> = vec![];
//     let j: Value = json!(self);
//     match j.as_object() {
//         Some(m) => {
//             for v in Self::get_table_order().iter() {
//                 let k: String = v.to_string();
//                 let val = m.get(&k).unwrap().clone();
//                 res.push(val);
//             }
//             Some(res)
//         }
//         None => None,
//     }
// }

pub fn get_th() -> Vec<(&'static str, &'static str)> {
    let th: Vec<(&str, &str)> = vec![
        ("dt", "일자"),
        ("why", "이유"),
        ("order_num", "순서"),
        ("line", "가격종류"),
        ("p1x", "시작일"),
        ("change_rate", "퍼센트"),
    ];
    th
}
