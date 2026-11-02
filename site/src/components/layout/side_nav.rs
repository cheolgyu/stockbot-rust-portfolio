use gloo::events::EventListener;

use yew::Component;

use crate::app::nav_state::{NavState, BP};
use crate::route::Route;

use yew::html::Scope;
use yew::prelude::*;
use yew_router::prelude::*;

pub enum Msg {
    ClickLink,
}
#[derive(Clone, PartialEq, Properties)]
pub struct Properties {
    pub state: NavState,
    pub cb: Callback<bool>,
}
pub struct SideNav {
    _listener: LocationHandle,
}
impl Component for SideNav {
    type Message = Msg;
    type Properties = Properties;

    fn create(ctx: &yew::Context<Self>) -> Self {
        let cb = ctx.link().callback(move |_| Msg::ClickLink);
        let listener: LocationHandle = ctx.link().add_location_listener(cb).unwrap();

        Self {
            _listener: listener,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ClickLink => {
                ctx.props().cb.emit(false);
            }
        }
        true
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let link = ctx.link();
        // let show_open_nav: String = if self.state.open_nav {
        //     String::from("SideNav  top-0 left-0 w-64 bg-white fixed h-full  ease-in-out transition-all duration-300 z-30 p-2  ")
        // } else {
        //     String::from("SideNav hidden md:flex  md:w-full ")
        // };
        let open_nav_class = ctx.props().state.cls_side_open();

        let p: String = ctx.link().location().unwrap().path().to_string();

        html! {
            <div class="">
                <nav class={open_nav_class} >
                    <div>
                            <h3>{ "바운드" }</h3>
                            <ul>
                                <li class=" ">
                                    <Link<Route>
                                    classes={classes!(if p == Route::Bound.to_path() { "active" }else{""})}
                                    to={Route::Bound}>
                                    { "바운드" }
                                </Link<Route>>
                            </li>
                            </ul>
                    </div>
                    <div>
                            <h3>{ "빈도 통계" }</h3>
                            <ul>
                                <li class=" ">
                                <Link<Route>
                                    classes={classes!(if p == Route::AggVol.to_path() { "active" }else{""})}
                                    to={Route::AggVol}>
                                    { "거래량 통계" }
                                </Link<Route>>
                            </li>
                            <li class=" ">
                                <Link<Route>
                                    classes={classes!(if p == Route::AggCp.to_path() { "active" }else{""})}
                                    to={Route::AggCp}>
                                    { "종가 통계" }
                                </Link<Route>>
                            </li>
                            <li class=" ">
                                <Link<Route>
                                    classes={classes!(if p == Route::AggBound.to_path() { "active" }else{""})}
                                    to={Route::AggBound}>
                                    { "바운드 통계" }
                                </Link<Route>>
                            </li>
                            </ul>
                    </div>
                    <div>
                            <h3>{ "비율" }</h3>
                            <ul>
                                <li class=" ">
                                    <Link<Route>
                                        classes={classes!(if p == Route::RateBtDate.to_path() { "active" }else{""})}
                                        to={Route::RateBtDate}>
                                        { "일별 거래량 비율" }
                                    </Link<Route>>
                                </li>
                                <li class=" ">
                                    <Link<Route>
                                        classes={classes!(if p == Route::PriceChg.to_path() { "active" }else{""})}
                                        to={Route::PriceChg}>
                                        { "가격 변화 " }
                                    </Link<Route>>
                                </li>
                            </ul>
                    </div>
                    <div>
                            <h3>{ "시스템" }</h3>
                            <ul>
                                <li class="">
                                <Link<Route>
                                    classes={classes!(if p == Route::SystemLogsBatch.to_path() { "active" }else{""})}
                                    to={Route::SystemLogsBatch}>
                                    { "배치" }
                                </Link<Route>>
                            </li>
                            <li class="">
                                <Link<Route>
                                    classes={classes!(if p == Route::Swagger.to_path() { "active" }else{""})}
                                    to={Route::Swagger}>
                                    { "Swagger" }
                                </Link<Route>>
                            </li>
                            </ul>
                    </div>
                    <div>
                            <h3>{ "설명서" }</h3>
                            <ul>
                                <li class="">
                                <Link<Route>
                                    classes={classes!(if p == Route::Manual.to_path() { "active" }else{""})}
                                    to={Route::Manual}>
                                    { "이용 설명서" }
                                </Link<Route>>
                            </li>
                            <li class="">
                                <Link<Route>
                                    classes={classes!(if p == Route::SystemManual.to_path() { "active" }else{""})}
                                    to={Route::SystemManual}>
                                    { "시스템 설명서" }
                                </Link<Route>>
                            </li>
                            </ul>
                    </div>
                    //////////////////////////////////////////////////////

                    ///////////////////////////////////////
                </nav>
            </div>
        }
    }
}
