use crate::{
    components::{
        paging::Paging,
        search_box::{
            exchange::checkbox::CheckBoxExchange,
            price_type::{radio::RadioPriceType, ElPriceTypes},
            search::input::InputSearch,
        },
    },
    request,
    route::Route,
    utils::{
        page::DefaultParam,
        thead_info::{td_cell, ColumnKind, TH_BOUND},
    },
};
use batch_model::{
    app_type::{exchange_type::ExchangeType, price_type::PriceType},
    bound::view::list::BoundViewList,
    site::{param::bound_list::Param, res::PageResult},
};
use gloo::console::error;
use log::info;
use serde_json::{json, Map, Value};
use web_sys::HtmlElement;
use yew::prelude::*;
use yew_router::scope_ext::RouterScopeExt;
//https://docs.rs/yew-router/latest/src/yew_router/components/link.rs.html#65
//https://github.com/yewstack/yew/blob/master/examples/router/src/pages/post_list.rs

type Res = PageResult<BoundViewList>;

pub enum Msg {
    //https://yew.rs/docs/concepts/suspense#render-as-you-fetch
    Fetch,
    FetchOk(Res),
    FetchErr,
    EventTh(MouseEvent),
    EventExchage(Vec<ExchangeType>),
    EventSearch(String),
    EventPriceType(PriceType),
    EventTr(MouseEvent),
    EventPage(usize),
}

#[derive(Debug)]
pub struct Bound {
    data: Res,
    param: Param,
    el_price_types: ElPriceTypes,
    show_col: Vec<usize>,
}

fn current_page(ctx: &Context<Bound>) -> (Param, DefaultParam) {
    let location = ctx.link().location().unwrap();
    // info!("current_page. 바운드목록 ");
    // info!("{:?}", location);
    location
        .query::<Param>()
        .map(|it| (it, DefaultParam::F))
        .unwrap_or((Param::default(), DefaultParam::T))
}

impl Component for Bound {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_future(async { Msg::Fetch });
        let (param, default_param) = current_page(ctx);
        let mut item = Self {
            data: Res::default(),
            param,
            el_price_types: ElPriceTypes::new(),
            show_col: vec![],
        };

        item.update_show_col(PriceType::CLOSE);
        item
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        //let mut param = &self.param;
        match msg {
            Msg::Fetch => {
                info!("Fetch. 바운드목록");
                    //self.set_query();
                    ctx.link()
                        .navigator()
                        .unwrap()
                        .replace_with_query(&Route::Bound, &self.param)
                        .unwrap();

                let param = self.param.clone();
                ctx.link().send_future(async {
                    match request::fetch(param).await {
                        Ok(page_result) => {
                            let res: Res = serde_json::from_value(page_result).unwrap();
                            Msg::FetchOk(res)
                        }
                        Err(err) => {
                            error!("page-오류찾기-1:{}", err.to_string());
                            Msg::FetchErr
                        }
                    }
                });
            }
            Msg::FetchOk(page_result) => {
                info!("FetchOk. 바운드목록");
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
            Msg::EventExchage(v) => {
                self.param.exchange.exchange = v;
                self.param.paging.page = 1;
                ctx.link().send_message(Msg::Fetch);
            }
            Msg::EventSearch(v) => {
                self.param.search.search = Some(v);
                ctx.link().send_message(Msg::Fetch);
            }
            Msg::EventPriceType(v) => {
                self.param.line.line = v;
                ctx.link().send_message(Msg::Fetch);
            }
            Msg::EventTr(e) => {
                let t = e.target_dyn_into::<HtmlElement>().unwrap();
                let tr = t.closest("tr").unwrap().unwrap();
                let code = tr.get_attribute("tr_code").unwrap();
                let navigator = ctx.link().navigator().unwrap();
                navigator.push(&Route::CodeDetail { code });
            }

            Msg::EventPage(v) => {
                self.param.paging.page = v;
                ctx.link().send_message(Msg::Fetch);
            }
        }
        true
    }
    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let tbody = self.tbody(ctx);
        let thead = self.thead(ctx);
        let cb_exchange = ctx.link().callback(Msg::EventExchage);
        let cb = ctx.link().callback(Msg::EventSearch);

        let info = self.data.page_info.clone();
        let cnt = format!("( {} 건)", self.data.page_info.total_cnt);
        let el_price_types = self.el_price_types.clone();
        let checked = self.param.line.line;

        let cb_pt = ctx.link().callback(Msg::EventPriceType);
        let jp: Map<String, Value> = json!(self.param).as_object().unwrap().clone();
        let link = ctx.link().clone();
        let cb_paging: Callback<usize> =
            Callback::from(move |v: usize| link.send_message(Msg::EventPage(v)));
        let ex = self.param.exchange.exchange.clone();
        html! {
            <>
            <div class="search_area">
                 <div class="search_area_first">
                    <div class="search_area_item "><RadioPriceType checked={checked} list={el_price_types} cb={cb_pt} ></RadioPriceType></div>
                    <div class="search_area_item" > <CheckBoxExchange default_checked_key={ex} {cb_exchange}></CheckBoxExchange></div>

                </div>
                <div>
                    <div class="search_area_item"><InputSearch  {cb} ></InputSearch></div>
                </div>

            </div>

            <div class="table_area">
                <table  >
                    <caption>{"바운드 목록"} {cnt} </caption>
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

impl Bound {
    fn update_show_col(&mut self, price_type: PriceType) {
        let res: Vec<usize> = vec![];
        let short_key = price_type.short_key().to_string();
        // for it in self.th_name.iter().enumerate() {
        //     if it.1.contains("p_") {
        //         let mut nm = it.1.clone();
        //         let _ = nm.split_off(2);
        //         if short_key == nm {
        //             res.push(it.0);
        //         }
        //     } else {
        //         // for col in &self.allow_col {
        //         //     if col == it.1 {
        //         //         res.push(it.0);
        //         //     }
        //         // }
        //     }
        // }

        self.show_col = res;
    }

    fn thead(&self, ctx: &Context<Self>) -> Html {
        let on_clicked = ctx.link().callback(self::Msg::EventTh);

        let th: Vec<(&str, &str)> = vec![("name", "이름")];

        let th_set1 = th
            .iter()
            .map(|(k, v)| {
                let value: String = self.param.paging.sort_val(k, v);
                html! {
                    <th class="cursor-pointer break-all" rowspan="3" click_id={*k} onclick={on_clicked.clone()} >{value}</th>
                }
            })
            .collect::<Html>();

        let th_set2 = html! {
            <th   colspan="11"  >{&self.param.line.line.to_text()}</th>
        };

        let th_set3 = TH_BOUND
                    .iter()
                    .map(|col| {
                        let value: String = self.param.paging.sort_val(col.key, col.label);
                        html! {
                            <th class="cursor-pointer"  click_id={col.key} onclick={on_clicked.clone()} >{value}</th>

                        }
                    })
            .collect::<Html>();

        html! {
            <thead class="">
                <tr>
                    {th_set1}
                    {th_set2}
                </tr>
                <tr>
                    <th colspan="8" >{"반등"}</th>
                    <th colspan="3" >{"ymxb"}</th>

                </tr>
                <tr>
                    {th_set3}
                </tr>
            </thead>
        }
    }

    fn tbody(&self, ctx: &Context<Self>) -> Html {
        let tr_list = self
            .data
            .data
            .iter()
            .map(|item: &BoundViewList| {
                let j = json!(item);
                let v = j.as_object().unwrap();

                let tr_td = TH_BOUND
                    .iter()
                    .map(|th_info| td_cell(&th_info.value_type, &v[th_info.key]))
                    .collect::<Html>();
                let on_clicked = ctx.link().callback(self::Msg::EventTr);

                let tr_code = v["code"].as_str().unwrap().to_string();

                html! {
                    <tr class="cursor-pointer" {tr_code}  onclick={on_clicked.clone()} >
                        {td_cell(&ColumnKind::NONE,&v["name"])}
                        {  tr_td }
                    </tr>
                }
            })
            .collect::<Html>();
        html! {
            <tbody>
                {tr_list}
            </tbody>
        }
    }
}
