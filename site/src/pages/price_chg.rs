use crate::{
    components::{
        paging::Paging,
        search_box::{
            exchange::checkbox::CheckBoxExchange, input_date::InputDate, search::input::InputSearch,
        },
    },
    request,
    route::Route,
    utils::thead_info::{td_cell, TH_VIEW_PRICE_CHG},
};
use batch_model::{
    app_type::exchange_type::ExchangeType,
    price_chg::view::ViewPriceChg,
    site::{param::view_price_chg_param::list::Param, res::PageResultDt},
};
use chrono::NaiveDate;
use log::info;
use serde_json::{json, Map, Value};

use web_sys::HtmlElement;
use yew::prelude::*;
use yew_router::scope_ext::{LocationHandle, RouterScopeExt};
//https://docs.rs/yew-router/latest/src/yew_router/components/link.rs.html#65
//https://github.com/yewstack/yew/blob/master/examples/router/src/pages/post_list.rs

type Res = PageResultDt<ViewPriceChg>;

pub enum Msg {
    //https://yew.rs/docs/concepts/suspense#render-as-you-fetch
    Fetch,
    FetchOk(Res),
    FetchErr,
    EventTh(MouseEvent),
    EventExchage(Vec<ExchangeType>),
    EventSearch(String),
    EventInputDate(NaiveDate),
    EventTr(MouseEvent),
    PageUpdatedParams,
    EventPage(usize),
}
type Page = PriceChg;
pub struct PriceChg {
    data: Res,
    params: Param,
}

fn current_page(ctx: &Context<Page>) -> Param {
    let location = ctx.link().location().unwrap();

    location.query::<Param>().unwrap_or_default()
}
/*

https://yew.rs/docs/concepts/router
Routers in Single Page Applications (SPA) handle displaying different pages depending on what the URL is.
Instead of the default behavior of requesting a different remote resource when a link is clicked,
the router instead sets the URL locally to point to a valid route in your application.
The router then detects this change and then decides what to render.
 */

impl Component for Page {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_future(async { Msg::Fetch });

        Self {
            data: Res::default(),
            params: current_page(ctx),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        //let mut params = &self.params;
        match msg {
            Msg::Fetch => {
                let params = self.params.clone();
                ctx.link().send_future(async {
                    match request::fetch(params).await {
                        Ok(page_result) => {
                            let res: Res = serde_json::from_value(page_result).unwrap();
                            Msg::FetchOk(res)
                        }
                        Err(_err) => Msg::FetchErr,
                    }
                });
            }
            Msg::FetchOk(page_result) => {
                self.params.dt = page_result.dt;
                self.data = page_result;
            }
            Msg::FetchErr => {
                info!("exe Msg::FetchErr");
            }
            Msg::EventTh(e) => {
                let t = e.target_dyn_into::<HtmlElement>().unwrap();
                let k = t.get_attribute("click_id").unwrap();
                self.params.paging.set_sort_mutil(k);
                //ctx.link().location().insert(value)
                //let loc = use_location()

                ctx.link().send_message(Msg::PageUpdatedParams);
            }
            Msg::EventExchage(v) => {
                self.params.exchange.exchange = v;
                ctx.link().send_message(Msg::PageUpdatedParams);
            }
            Msg::EventSearch(v) => {
                self.params.search.search = Some(v);
                ctx.link().send_message(Msg::PageUpdatedParams);
            }
            Msg::EventInputDate(v) => {
                self.params.dt = Some(v);
                ctx.link().send_message(Msg::PageUpdatedParams);
            }
            Msg::EventTr(e) => {
                let t = e.target_dyn_into::<HtmlElement>().unwrap();
                let tr = t.closest("tr").unwrap().unwrap();
                let code = tr.get_attribute("tr_code").unwrap();
                let navigator = ctx.link().navigator().unwrap();
                navigator.push(&Route::CodeDetail { code });
            }

            Msg::PageUpdatedParams => {
                let navigator = ctx.link().navigator().unwrap();

                navigator
                    .push_with_query(&Route::PriceChg, &self.params)
                    .unwrap();
                ctx.link().send_message(Msg::Fetch);
            }
            Msg::EventPage(v) => {
                self.params.paging.page = v;
                ctx.link().send_message(Msg::PageUpdatedParams);
            }
        }
        true
    }

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let tbody = self.tbody(ctx);
        let thead = self.thead(ctx);
        let cb_exchange = ctx.link().callback(Msg::EventExchage);
        let cb_search = ctx.link().callback(Msg::EventSearch);
        let cb_date = ctx.link().callback(Msg::EventInputDate);

        let info = self.data.page_info.clone();
        let cnt = format!("( {} 건)", self.data.page_info.total_cnt);
        let jp: Map<String, Value> = json!(self.params).as_object().unwrap().clone();
        let ex = self.params.exchange.exchange.clone();

        let link = ctx.link().clone();
        let cb_paging: Callback<usize> =
            Callback::from(move |v: usize| link.send_message(Msg::EventPage(v)));

        html! {
            <>
            <div class="search_area">
                <div class="search_area_item " > <CheckBoxExchange default_checked_key={ex}  {cb_exchange}></CheckBoxExchange></div>
                <div class="search_area_item">
                    <InputDate cb={cb_date} default={self.params.dt}></InputDate>
                </div>

                <div class="search_area_item "><InputSearch  cb={cb_search} ></InputSearch></div>
                <div class="search_area_item "></div>
            </div>

            <hr/>
            <div class="table_area">
                <table >
                    <caption>{"가격 변화 목록"} {cnt} </caption>
                    {thead}
                    {tbody}
                </table>
            </div>

            <div class="paging_area">
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

impl Page {
    fn thead(&self, ctx: &Context<Self>) -> Html {
        let on_clicked = ctx.link().callback(self::Msg::EventTh);
        let th_set1 = TH_VIEW_PRICE_CHG
            .iter()
            .map(|thead_info| {
                let value: String = self.params.paging.sort_val(thead_info.key, thead_info.label);
                html! {
                    <th class="cursor-pointer" click_id={thead_info.key} onclick={on_clicked.clone()} >{value}</th>
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

    fn tbody(&self, ctx: &Context<Self>) -> Html {
        let td_set1 = self
            .data
            .data
            .iter()
            .map(|item| {
                let j = json!(item);
                let v = j.as_object().unwrap();
                let on_clicked = ctx.link().callback(self::Msg::EventTr);
                let tr_code = v["code"].as_str().unwrap().to_string();

                let tr_td = TH_VIEW_PRICE_CHG
                    .iter()
                    .map(|th_info| td_cell(&th_info.value_type, &v[th_info.key]))
                    .collect::<Html>();

                html! {
                    <tr class="cursor-pointer" {tr_code}  onclick={on_clicked.clone()} >
                        {  tr_td }
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
