use gloo::events::EventListener;

use yew::Component;

use crate::app::nav_state::{NavState, BP};
use crate::route::Route;

use yew::html::Scope;
use yew::prelude::*;
use yew_router::prelude::*;

pub enum Msg {
    NavBarToggle,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Properties {
    pub state: NavState,
    pub cb: Callback<bool>,
}
pub struct NavBar {}
impl Component for NavBar {
    type Message = Msg;
    type Properties = Properties;

    fn create(ctx: &yew::Context<Self>) -> Self {
        //

        Self {}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::NavBarToggle => {
                ctx.props().cb.emit(true);
            }
        }
        true
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let link = ctx.link();
        let p: String = ctx.link().location().unwrap().path().to_string();

        html! {
                <nav class="    flex justify-between  items-center " >

                    <div class="flex w-32 font-black text-2xl">
                        <Link<Route> classes={classes!("navbar-brand")} to={Route::Home}>
                            { "주식봇" }
                        </Link<Route>>
                    </div>
                    <div class="invisible max-lg:visible  order-last font-semibold  text-lg" >
                        <button  onclick={link.callback(|_| Msg::NavBarToggle)} >
                            if ctx.props().state.open_nav  {
                                // x
                                <svg class="w-10" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" >
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" />
                                </svg>

                            } else {
                                // menu
                                <svg class="w-10 " xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" >
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5" />
                                </svg>
                            }
                        </button>
                    </div>
                </nav>
        }
    }
}
