use batch_model::site::res::{ResultPagingInfo, HORIZONTAL_ELLIPSIS};
use serde_json::{json, Map, Value};
use web_sys::HtmlElement;
use yew::prelude::*;

const ATTR_PAGE_NUM: &str = "to_page";
#[derive(Debug)]
pub struct Paging {}
#[derive(Clone, PartialEq, Properties)]
pub struct Properties {
    pub info: ResultPagingInfo,
    pub cb: Callback<usize>,
    pub params: Map<String, Value>,
}
pub enum Msg {}
/*
   프로퍼티로 라우터를 받으면 간편하지만
   한개의 페이지 안에 url이 고정이면서 페이징을 하게되는 경우인
   code/:code 안에서의 가격목록 페이징 처리를 위해 다시 콜백으로 변경...
*/
impl Component for Paging {
    type Message = Msg;
    type Properties = Properties;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        //debug!("changed. 페이징컴포넌트 ");
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props().clone();
        let info = &props.info;
        let cb_props = props.cb.clone();
        let cb = Callback::from(move |e: MouseEvent| {
            let t = e
                .target_dyn_into::<HtmlElement>()
                .unwrap()
                .get_attribute(ATTR_PAGE_NUM)
                .unwrap();
            let p = t.parse::<usize>().unwrap();
            cb_props.clone().emit(p);
        });
        let params = props.params.clone();

        let show = info.total_cnt > 0;

        let li = self.view_li(ctx);
        let li_1 = info.cur > HORIZONTAL_ELLIPSIS + 1;
        let mut li_1_p = params.clone();
        li_1_p.insert("page".to_string(), json!(1));
        let chk = info.max.checked_sub(info.cur).is_some();
        let li_max = chk && ((info.max - info.cur) > HORIZONTAL_ELLIPSIS) && info.cur != info.max;
        let mut li_max_p = params.clone();
        li_max_p.insert("page".to_string(), json!(info.max));

        let dot_start = info.cur > HORIZONTAL_ELLIPSIS + 3;
        let dot_end = chk && (info.max - info.cur) > HORIZONTAL_ELLIPSIS;

        html! {
            <>

            if show {
                <nav class="p-2 border-solid border-2 border-sky-500 hover:border-dotted">
                    <ul class=" flex space-x-4  mx-auto " >
                        if li_1{
                            <li class="paging-li"  >
                                { self.view_paging_li(1_usize,"처음".to_string(), cb.clone() )}
                            </li>
                        }
                        if dot_start {
                            <li class="paging-li" >{"…"}</li>
                        }
                        {li}
                        if dot_end {
                            <li class="paging-li" >{"…"}</li>
                        }
                        if li_max{
                            <li class="paging-li"  >
                                { self.view_paging_li(info.max,"마지막".to_string(), cb.clone() )}
                            </li>
                        }
                    </ul>
                </nav>

            }
            </>



        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {
        //debug!("재랜더링. 페이징컴포넌트 ");
    }

    fn prepare_state(&self) -> Option<String> {
        None
    }

    fn destroy(&mut self, _ctx: &Context<Self>) {}
}

impl Paging {
    // https://yew.rs/docs/concepts/contexts#using-contexts
    // https://yew.rs/docs/advanced-topics/struct-components/hoc#example
    fn view_li(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props().clone();
        let info = &props.info;
        let cb_props = props.cb;
        let cb = Callback::from(move |e: MouseEvent| {
            let t = e
                .target_dyn_into::<HtmlElement>()
                .unwrap()
                .get_attribute(ATTR_PAGE_NUM)
                .unwrap();
            let p = t.parse::<usize>().unwrap();
            cb_props.clone().emit(p);
        });
        let paging = info.paging.clone();

        let coll = paging
            .iter()
            .map(|p| {
                let cur = info.cur == *p;
                let to_page = *p;
                let mut params = ctx.props().params.clone();
                params.insert("page".to_string(), json!(to_page));

                html! {
                    <>
                        <li  class={classes!("paging-li", if cur{  "paging-active "} else{""})}  >
                            { self.view_paging_li(to_page,to_page.to_string(), cb.clone() )}
                        </li>
                    </>
                }
            })
            .collect::<Html>();
        coll
    }

    pub fn view_paging_li(
        &self,
        to_page: usize,
        to_page_text: String,
        cb: Callback<MouseEvent>,
    ) -> Html {
        html! {
            <a to_page={format!("{}",to_page)} onclick={cb}> {to_page_text} </a>
        }
    }
}
