use batch_model::{
    log::batch::LogBatch,
    site::res::PageResult,
};

use yew::prelude::*;

type Res = PageResult<LogBatch>;
pub enum Msg {}

#[derive(Clone, PartialEq, Properties)]
pub struct Properties<T: std::cmp::PartialEq> {
    pub cols: Vec<String>,
    pub rows: Vec<T>,
}

pub struct TbGrid {}

impl Component for TbGrid {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        // ctx.link().send_future(async { Msg::Fetch });
        Self {}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        true
    }

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <h1> {"TbGrid"} </h1>
            </>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {}

    fn destroy(&mut self, _ctx: &Context<Self>) {}
}
