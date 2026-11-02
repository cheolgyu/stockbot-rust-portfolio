use log::error;
use serde_json::Value;
use yew::prelude::*;

use crate::{impl_param::param_batch_timer_health, request};

pub enum Msg {
    Fetch,
    FetchOk(Value),
    FetchErr,
}

pub struct Home {
    pub health: String,
}
impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let health = "".to_string();
        Self { health }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
        <>
            <div class="grid grid-flow-row-dense grid-cols-1 grid-rows-1 ">
                <div class="col"></div>
                <div class="col">
                    <p class="font-mono text-5xl" >
                        {" 쌀때 사서 비쌀때 판다 "}


                    </p>
                </div>

                <div class="col"> {"ping.batch_exe="}{self.health.clone()} </div>
       
            </div>
        </>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Fetch => {
                ctx.link().send_future(async {
                    let param = param_batch_timer_health::Param {};
                    match request::fetch(param).await {
                        Ok(page_result) => Msg::FetchOk(page_result),
                        Err(err) => {
                            error!("page-오류찾기-12223:{}", err);
                            Msg::FetchErr
                        }
                    }
                });
            }
            Msg::FetchOk(page_result) => {
                self.health = page_result.as_str().unwrap().to_string();
            }
            Msg::FetchErr => {
                error!("exe Msg::FetchErr");
            }
        }
        true
    }

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        true
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            ctx.link().send_future(async { Msg::Fetch });
        }
    }

    fn prepare_state(&self) -> Option<String> {
        None
    }

    fn destroy(&mut self, _ctx: &Context<Self>) {}
}
