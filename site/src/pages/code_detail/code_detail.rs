use batch_model::{code::view::list::CodeViewList, site::res::PageResultOne};
use gloo::console::info;
use yew::prelude::*;

use crate::{
    impl_param::param_by_code_code::Param,
    pages::code_detail::{
        agg_bound_list::AggBoundList, agg_cp_list::AggCpList, agg_vol_list::AggVolList,
        bound_list::BoundList, price_chg_list::PriceChgList, price_list::PriceList,
        rate_by_date::RateByDateList,
    },
    request::{self},
};

type Res = PageResultOne<CodeViewList>;

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props {
    pub code: String,
}

pub enum Msg {
    //https://yew.rs/docs/concepts/suspense#render-as-you-fetch
    Fetch,
    FetchOk(Res),
    FetchErr,
}

pub struct CodeDetail {
    pub short_code: String,
    pub ready: bool,
    pub param: Param,
    pub data_code: Res,
}
impl Component for CodeDetail {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let short_code = ctx.props().code.clone();
        ctx.link().send_message(Msg::Fetch);

        let param = Param {
            short_code: short_code.clone(),
        };
        Self {
            short_code,
            param,
            data_code: Res::default(),
            ready: false,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let fallback = html! {<div>{"Loading..."}</div>};

        html! {
             <Suspense {fallback}>
                <hr/>
                    {self.view_data_code(ctx)}
                <hr/>
                <div class="code_detail_div">
                    <div class="data"> {self.view_content(ctx)} </div>
                    <div class="page_info">
                        <div>
                            <h3>  {"기본"} </h3>
                            <ul>
                                <li>
                                    <a href="#price"> {"가격 목록"} </a>
                                </li>
                                <li>
                                    <a href="#bound"> {"바운드 목록"} </a>
                                </li>
                            </ul>
                        </div>
                        <div>
                            <h3>  {"빈도 "} </h3>
                            <ul>
                                <li>
                                    <a href="#agg_vol"> {"거래량"} </a>
                                </li>
                                <li>
                                    <a href="#agg_cp"> {"종가"} </a>
                                </li>
                                <li>
                                    <a href="#agg_bound"> {"바운스"} </a>
                                </li>
                            </ul>
                        </div>
                        <div>
                            <h3>  {"비중 "} </h3>
                            <ul>
                                <li>
                                    <a href="#rate_by_date"> {"일별"} </a>
                                </li>
                                <li>
                                    <a href="#price_chg"> {"가격 변화"} </a>
                                </li>
                            </ul>
                        </div>
                    </div>
                </div>

            </Suspense>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
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
                false
            }
            Msg::FetchOk(page_result) => {
                info!("FetchOk. 바운드목록");
                self.data_code = page_result;
                self.ready = true;
                true
            }
            Msg::FetchErr => {
                info!("exe Msg::FetchErr");
                false
            }
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        true
    }

    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {}

    fn prepare_state(&self) -> Option<String> {
        None
    }

    fn destroy(&mut self, _ctx: &Context<Self>) {}
}
impl CodeDetail {
    fn view_content(&self, _ctx: &Context<Self>) -> Html {
        let code_id = self.data_code.data.id;
        html! {
            <>
                if self.ready {
                     <PriceList code_id={code_id}></PriceList>
                     <br/>
                     <br/>
                     <BoundList short_code={self.short_code.clone()} code_id={code_id}></BoundList>
                     <br/>
                     <br/>
                     <AggVolList code={self.short_code.clone()} code_id={code_id}></AggVolList>
                     <br/>
                     <br/>
                     <AggBoundList code={self.short_code.clone()} code_id={code_id}></AggBoundList>
                     <br/>
                     <br/>
                     <AggCpList code={self.short_code.clone()} code_id={code_id}></AggCpList>
                     <br/>
                     <br/>
                     <RateByDateList code={self.short_code.clone()} code_id={code_id}></RateByDateList>

                     <br/>
                     <br/>
                     <PriceChgList code={self.short_code.clone()} code_id={code_id}></PriceChgList>
                     <br/>
                     <br/>

                    // <FindList code={self.short_code.clone()} code_id={code_id}></FindList>
                }

            </>

        }
    }

    fn view_data_code(&self, _ctx: &Context<Self>) -> Html {
        let name = &self.data_code.data.short_name;
        let code = &self.data_code.data.short_code;
        let exchange = &self.data_code.data.exchange;
        let link_naver = "네이버로 이동".to_string();
        let link_naver_herf = format!("https://finance.naver.com/item/main.naver?code={}", &code);
        html! {
            <>
                <p>
                    <span class="text-2xl p-2">
                        { format!("{}",name) }{ format!("({})",code) }
                    </span>
                    <span class="text-xl p-2">
                        { format!("{}",exchange) }
                    </span>
                    <span class="text-lg  p-2">
                        <a target="_blank" href={link_naver_herf}>{link_naver}</a>
                    </span>
                </p>



            </>

        }
    }
}
