use crate::{
    components::paging::Paging,
    request,
    utils::{
        fmt::{tb_num::td_num, td::td},
        thead_info::{td_cell, ColumnKind},
    },
};
use batch_model::{
    price::PriceView,
    site::{param::price_by_code::Param, res::PageResult},
};
use log::info;
use serde_json::{json, Map, Value};
use web_sys::HtmlElement;
use yew::prelude::*;
//https://docs.rs/yew-router/latest/src/yew_router/components/link.rs.html#65
//https://github.com/yewstack/yew/blob/master/examples/router/src/pages/post_list.rs

type Res = PageResult<PriceView>;

pub enum Msg {
    //https://yew.rs/docs/concepts/suspense#render-as-you-fetch
    Fetch,
    FetchOk(Res),
    FetchErr,
    EventTh(MouseEvent),
    EventPage(usize),
}
#[derive(Clone, PartialEq, Properties)]
pub struct Properties {
    pub code_id: i64,
}
pub struct PriceList {
    param: Param,
    data: Res,
}

/*

https://yew.rs/docs/concepts/router
Routers in Single Page Applications (SPA) handle displaying different pages depending on what the URL is.
Instead of the default behavior of requesting a different remote resource when a link is clicked,
the router instead sets the URL locally to point to a valid route in your application.
The router then detects this change and then decides what to render.
 */

impl Component for PriceList {
    type Message = Msg;
    type Properties = Properties;

    fn create(ctx: &Context<Self>) -> Self {
        let mut param = Param::default();
        param.code_id = ctx.props().code_id;

        Self {
            data: Res::default(),
            param,
        }
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
                self.param.paging.set_sort_mutil(k);
                ctx.link().send_message(Msg::Fetch);
            }
            Msg::EventPage(v) => {
                self.param.paging.page = v;
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

        html! {
            <>
            <hr/>
            <h1 id="price">{ "가격 목록..." }</h1>
            <hr/>
            <div class="table_area">
                <table  >
                    <caption>{"가격 목록"} {cnt} </caption>
                    {thead}
                    {tbody}
                </table>
            </div>

            <div class="paging_area ">
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
    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            ctx.link().send_message(Msg::Fetch);
        }
    }

    fn prepare_state(&self) -> Option<String> {
        None
    }

    fn destroy(&mut self, _ctx: &Context<Self>) {}
}

impl PriceList {
    fn thead(&self, ctx: &Context<Self>) -> Html {
        let on_clicked = ctx.link().callback(self::Msg::EventTh);

        let th: Vec<(&str, &str)> = vec![
            ("dt", "일자"),
            //("dt_point_id", "일자 좌표값"),
            ("cp", "종가"),
            ("op", "시가"),
            ("lp", "저가"),
            ("hp", "고가"),
            // ("op_point", "시가 좌표값"),
            // ("hp_point", "고가 좌표값"),
            // ("lp_point", "저가 좌표값"),
            // ("cp_point", "종가 좌표값"),
            ("vol", "거래량"),
        ];

        let th_set1 = th
            .iter()
            .map(|(k, v)| {
                let value = self.param.paging.sort_val(k, v);
                html! {
                    <th class="cursor-pointer"
                        click_id={*k}
                        onclick={on_clicked.clone()} >{value}</th>
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
                let j = json!(item);
                let v = j.as_object().unwrap();

                html! {
                    <tr   >
                        {td_cell(&ColumnKind::TEXT ,&v["dt"])}
                        {td_cell(&ColumnKind::PRICE ,&v["cp"])}
                        {td_cell(&ColumnKind::PRICE ,&v["op"])}
                        {td_cell(&ColumnKind::PRICE ,&v["lp"])}
                        {td_cell(&ColumnKind::PRICE ,&v["hp"])}
                        {td_cell(&ColumnKind::PRICE ,&v["vol"])}

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
