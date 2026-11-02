use std::str::FromStr;

use crate::{
    components::{
        paging::Paging,
        search_box::{
            el_select::{ElOptions, ElSelect},
            exchange::checkbox::CheckBoxExchange,
            search::input::InputSearch,
        },
    },
    request::{self},
    route::Route,
    utils::thead_info::{td_cell, ColumnKind},
};

use batch_model::{
    app_type::{exchange_type::ExchangeType, find_what::What, find_why::Why},
    find::view::list::FindViewList,
    site::{param::find_list::Param, res::PageResultFind},
};
use log::{debug, error, info};

use serde_json::{json, Map, Value};

use web_sys::HtmlElement;
use yew::prelude::*;
use yew_router::scope_ext::RouterScopeExt;
//https://docs.rs/yew-router/latest/src/yew_router/components/link.rs.html#65
//https://github.com/yewstack/yew/blob/master/examples/router/src/pages/post_list.rs

type Res = PageResultFind<FindViewList>;

pub enum Msg {
    //https://github.com/yewstack/yew/blob/master/examples/futures/src/main.rs#L61
    //https://github.com/yewstack/yew/blob/a32442a3ded5bc53cd247ceeae99f884b11e57f3/examples/futures/src/main.rs#L61
    //https://yew.rs/docs/concepts/suspense#render-as-you-fetch
    Fetch,
    FetchOk(Res),
    FetchErr,
    EventTr(MouseEvent),
    EventExchage(Vec<ExchangeType>),
    EventSearch(String),
    EventSelect(String),
    SetQuery,
    EventPage(usize),
}
#[derive(Clone, PartialEq, Properties)]
pub struct Properties {
    #[prop_or_default]
    pub what: String,
    pub route: Route,
}

pub struct TbFind {
    data: Res,
    selectbox_option: ElOptions,
    param: Param,
}

fn empty_query(ctx: &Context<TbFind>) {
    let mut empty_query = false;
    if ctx.link().location().unwrap().query_str() == "" {
        empty_query = true;
    }

    if empty_query {
        debug!("empty_query ==> exec Msg::SetQuery)  : {:?}", empty_query);
    } else {
        debug!(
            "empty_query ==>query: {:?}",
            ctx.link().location().unwrap().query_str()
        );
    }
}

impl Component for TbFind {
    type Message = Msg;
    type Properties = Properties;

    fn create(ctx: &Context<Self>) -> Self {
        empty_query(ctx);
        let _link = ctx.link().clone();
        ctx.link().send_message(Msg::SetQuery);
        let mut p = Param::default();
        p.what = What::from_str(&ctx.props().what.clone()).unwrap();

        Self {
            data: Res::default(),
            param: p,
            selectbox_option: ElOptions(vec![]),
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
                        Err(err) => {
                            error!("page-오류찾기-1:{}", err);
                            Msg::FetchErr
                        }
                    }
                });
                false
            }
            Msg::FetchErr => {
                info!("exe Msg::FetchErr");
                true
            }
            Msg::EventTr(e) => {
                let t = e.target_dyn_into::<HtmlElement>().unwrap();
                let tr = t.closest("tr").unwrap().unwrap();
                let code = tr.get_attribute("tr_code").unwrap();
                let navigator = ctx.link().navigator().unwrap();
                navigator.push(&Route::CodeDetail { code });
                false
            }
            Msg::SetQuery => {
                debug!("Msg::SetQuery");
                let navigator = ctx.link().navigator().unwrap();
                let route = &ctx.props().route;
                navigator.push_with_query(route, &self.param).unwrap();
                ctx.link().send_message(Msg::Fetch);
                false
            }
            Msg::EventExchage(v) => {
                self.param.exchange.exchange = v;
                self.param.paging.page(1);
                ctx.link().send_message(Msg::SetQuery);
                false
            }
            Msg::EventSearch(v) => {
                self.param.search.search = Some(v);
                ctx.link().send_message(Msg::SetQuery);
                false
            }
            Msg::EventSelect(v) => {
                self.param.why = Why::from_str(v.as_str()).unwrap();
                ctx.link().send_message(Msg::SetQuery);
                false
            }
            Msg::EventPage(v) => {
                self.param.paging.page = v;
                ctx.link().send_message(Msg::SetQuery);
                false
            }

            Msg::FetchOk(page_result) => {
                self.data = page_result;
                let mut list: Vec<(String, String)> = vec![];
                for v in self.data.search_info.iter() {
                    let text = format!("{} ({}건)", v.0.text(), v.1);
                    list.push((v.0.to_string(), text));
                }
                info!(" -----------------> list = {:?}", list);
                self.selectbox_option = ElOptions(list);

                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let cnt = format!("( {} 건)", self.data.page_info.total_cnt);

        let jp: Map<String, Value> = json!(self.param).as_object().unwrap().clone();

        let info = self.data.page_info.clone();
        let ex = self.param.exchange.exchange.clone();
        let cb_exchange = ctx.link().callback(Msg::EventExchage);
        let cb = ctx.link().callback(Msg::EventSearch);
        let cb_select = ctx.link().callback(Msg::EventSelect);

        let tbody = self.tbody(ctx);
        let thead = self.thead(ctx);
        let link = ctx.link().clone();
        let cb_paging: Callback<usize> =
            Callback::from(move |v: usize| link.send_message(Msg::EventPage(v)));

        html! {
            <>
            <hr/>
            <div class="search_area">
                <div class="p-2 " > <CheckBoxExchange default_checked_key={ex} {cb_exchange}></CheckBoxExchange></div>
                <div class="p-2 "><InputSearch  {cb} ></InputSearch></div>
                <div class="p-2 " >
                    <ElSelect
                        default={Why::default().to_string()}
                        options={self.selectbox_option.clone()}
                        cb={cb_select} >
                    </ElSelect>
                </div>
            </div>
            <hr/>
            <div class="table_area">
                <table  >
                    <caption>{"파인트 목록"} {cnt}  </caption>
                    {thead}
                    {tbody}
                </table>
            </div>
            <div class="paging_area">
                <Paging
                    info={info}
                    cb={cb_paging}
                    params={jp.clone()}
                 ></Paging>
            </div>


            </>
        }
    }

    fn prepare_state(&self) -> Option<String> {
        None
    }

    fn destroy(&mut self, _ctx: &Context<Self>) {}
}

impl TbFind {
    fn thead(&self, _ctx: &Context<Self>) -> Html {
        let th: Vec<(&str, &str)> = vec![
            ("short_code", "코드"),
            ("short_name", "이름"),
            ("dt", "일자"),
            ("why", "이유"),
            ("order_num", "순서"),
            ("line", "라인"),
            ("change_rate", "change_rate"),
        ];

        let th_set1 = th
            .iter()
            .map(|(k, v)| {
                html! {
                    <th click_id={*k}  >{v}</th>
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
                let why = &Value::String(item.why.text());
                let line = &Value::String(item.line.to_text().to_string());
                let j = json!(item);
                let v = j.as_object().unwrap();

                let on_clicked = ctx.link().callback(self::Msg::EventTr);
                let tr_code = v["short_code"].as_str().unwrap().to_string();
                html! {
                    <tr class="cursor-pointer" {tr_code}  onclick={on_clicked.clone()} >
                        {td_cell(&ColumnKind::TEXT,&v["short_code"])}
                        {td_cell(&ColumnKind::TEXT,&v["short_name"])}

                        {td_cell(&ColumnKind::TEXT,&v["dt"])}
                        {td_cell(&ColumnKind::TEXT,why)}
                        {td_cell(&ColumnKind::NUMBER,&v["order_num"])}

                        {td_cell(&ColumnKind::TEXT, line)}
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
