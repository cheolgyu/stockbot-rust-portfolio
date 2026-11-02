use crate::{components::paging::Paging, request};
use batch_model::{
    batch_timer::BatchTimerPlan,
    site::{param::batch_timer_plan::Param, res::PageResult},
};
use gloo::console::error;
use log::debug;
use serde_json::{json, Map, Value};

use web_sys::HtmlElement;
use yew::prelude::*;

type Res = PageResult<BatchTimerPlan>;
type CurComponent = TbBatchTimerPlan;

pub enum Msg {
    Fetch,
    FetchOk(Res),
    FetchErr,
    EventPage(usize),
    EventTh(MouseEvent),
}

pub struct TbBatchTimerPlan {
    data: Res,
    param: Param,
}

impl Component for CurComponent {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_future(async { Msg::Fetch });

        Self {
            data: Res::default(),
            param: Param::default(),
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

            Msg::EventPage(v) => {
                self.param.paging.page = v;
                ctx.link().send_message(Msg::Fetch);
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
                <table>
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
           ("id","id",),              
            ("batch_type","batch_type",),              
            ("country_type",  "country_type",  ),              
            ("run_type",  "run_type",  ),              
            ("updated_dt","updated_dt",),              
            ("crated_dt", "crated_dt", ),              
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
                        {td("",&v["id"])}
                        {td("",&v["batch_type"])}
                        {td("",&v["country_type"])}
                        {td("",&v["run_type"])}
                        {td("",&v["updated_dt"])}
                        {td("",&v["crated_dt"])}
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
