use crate::components::table::{tb_batch_log::TbBatchLog, tb_batch_timer_plan::TbBatchTimerPlan};

use yew::prelude::*;

pub enum Msg {}

pub struct LogBatch {}

impl Component for LogBatch {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
         <>
            <TbBatchTimerPlan></TbBatchTimerPlan>
            <TbBatchLog></TbBatchLog>
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
