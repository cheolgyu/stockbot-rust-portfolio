use crate::{components::paging::Paging, request, route::Route};
use batch_model::{
    log::batch::LogBatch,
    site::{param::log_batch::Param, res::PageResult},
};
use gloo::console::error;
use log::debug;
use serde_json::{json, Map, Value};

use web_sys::HtmlElement;
use yew::prelude::*;
use yew_router::scope_ext::{LocationHandle, RouterScopeExt};

type Res = PageResult<LogBatch>;
pub enum Msg {
    Fetch,
    FetchOk(Res),
    FetchErr,
    PageUpdatedParams,
    EventPage(usize),
    EventTh(MouseEvent),
}

type CurComponent = TbBatchLog;

pub struct TbBatchLog {
    data: Res,
    param: Param,
}

fn current_page(ctx: &Context<CurComponent>) -> Param {
    let location = ctx.link().location().unwrap();
    location.query::<Param>().unwrap_or_default()
}

impl Component for CurComponent {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
       ctx.link().send_future(async { Msg::Fetch });
      

        Self {
            data: Res::default(),
            param: current_page(ctx),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        //let mut params = &self.params;
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
                            error!("page-오류찾기-1:{}", err.to_string());
                            Msg::FetchErr
                        }
                    }
                });
            }
            Msg::FetchOk(page_result) => {
                debug!("FetchOk. 바운드목록");
                self.data = page_result;
            }
            Msg::FetchErr => {
                error!("exe Msg::FetchErr");
            }
            Msg::PageUpdatedParams => {
                let navigator = ctx.link().navigator().unwrap();

                navigator
                    .push_with_query(&Route::SystemLogsBatch, &self.param)
                    .unwrap();  
                ctx.link().send_message(Msg::Fetch);
            }
            Msg::EventPage(v) => {
                self.param.paging.page = v;
                ctx.link().send_message(Msg::PageUpdatedParams);
            }
            Msg::EventTh(mouse_event) => {
                let t = mouse_event.target_dyn_into::<HtmlElement>().unwrap();
                let k = t.get_attribute("click_id").unwrap();
                self.param.paging.set_sort_mutil(k);

                ctx.link().send_message(Msg::Fetch);
            }
        }
        true
    }

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let tbody = self.tbody(ctx);
        let thead = self.thead(ctx).clone();
        let info = self.data.page_info.clone();
        let cnt = format!("( {} 건)", self.data.page_info.total_cnt);
        let jp: Map<String, Value> = json!(self.param).as_object().unwrap().clone();
        let link = ctx.link().clone();
        let cb_paging: Callback<usize> =
            Callback::from(move |v: usize| link.send_message(Msg::EventPage(v)));

        html! {
            <>


            <hr/>
            <div class="table_area">
                <table  >
                    <caption>{"배치 목록"} {cnt} </caption>
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
    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
      
    }

    fn prepare_state(&self) -> Option<String> {
        None
    }

    fn destroy(&mut self, _ctx: &Context<Self>) {}
}

impl CurComponent {
     fn thead(&self, ctx: &Context<Self>) -> Html {
        let on_clicked = ctx.link().callback(self::Msg::EventTh);

        let th: Vec<(&str, &str)> = vec![
            ("batch_id",         "배치 id"         ),
            ("step_id",         "배치단계 id"      ),
            ("step_job_name",     "배치단계 job이름" ),
            ("step_name",         "배치단계 step이름"),
            ("step_run_time_sec", "배치단계 실행시간"),
            ("step_start_time",   "배치단계 시작시간"),
            ("step_end_time",     "배치단계 종료시간")
        ];
        let th_set = th 
            .iter()
            .map(|th_info| {
                let value = self.param.paging.sort_val(th_info.0, th_info.1);
                html! {
                    <th class="cursor-pointer"  click_id={th_info.0} onclick={on_clicked.clone()} >{value}</th>
                }
            })
            .collect::<Html>();

        html! {
            <thead class="">
                <tr>
                     {th_set}
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
                        {td("",&v["batch_id"])}
                        // {td("",&v["batch_start_time"])}
                        // {td("",&v["batch_end_time"])}
                        // {td("",&v["batch_run_time_sec"])}
                        {td("",&v["step_id"])}
                        {td("",&v["step_job_name"])}
                        {td("",&v["step_name"])}
                        {td("",&v["step_run_time_sec"])}
                        {td("",&v["step_start_time"])}
                        {td("",&v["step_end_time"])}


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
