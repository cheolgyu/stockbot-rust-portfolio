use yew::prelude::*;
use yew_router::prelude::*;

use crate::{
    app::nav_state::{self, NavState},
    pages::{
        agg_bound::AggBound, agg_cp::AggCp, agg_vol::AggVol, bound::Bound,
        code_detail::code_detail::CodeDetail, find::*, home::Home, manual::Manual,
        page_not_found::PageNotFound, price_chg::PriceChg, rate_by_dt::RateBtDatePage,
        swagger::Swagger, system::LogBatch, system_manual::SystemManual,
    },
};

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/code/:code")]
    CodeDetail { code: String },

    #[at("/manual")]
    Manual,

    #[at("/rate_by_dt")]
    RateBtDate,
    #[at("/price_chg")]
    PriceChg,

    #[at("/agg/bound")]
    AggBound,
    #[at("/agg/vol")]
    AggVol,
    #[at("/agg/cp")]
    AggCp,
    #[at("/board")]
    Bound,

    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,

    // find
    #[at("/head")]
    FindHead,
    #[at("/shoulder")]
    FindShoulder,
    #[at("/knee")]
    FindKnee,
    #[at("/foot")]
    FindFoot,
    #[at("/swagger")]
    Swagger,
    #[at("/system/logs/batch")]
    SystemLogsBatch,
    #[at("/system/manual")]
    SystemManual,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::PriceChg => {
            html! { <PriceChg /> }
        }
        Route::Manual => {
            html! { <Manual /> }
        }
        Route::CodeDetail { code } => {
            html! { <CodeDetail code={code} /> }
        }
        Route::AggVol => {
            html! { <AggVol /> }
        }
        Route::AggBound => {
            html! { <AggBound /> }
        }
        Route::AggCp => {
            html! { <AggCp /> }
        }
        Route::Bound => {
            html! { <Bound /> }
        }
        Route::Home => {
            html! { <Home /> }
        }
        Route::NotFound => {
            html! { <PageNotFound /> }
        }
        Route::FindHead => {
            html! { <FindHead what={"H".to_string()} route={Route::FindHead} /> }
        }
        Route::FindShoulder => {
            html! { <FindShoulder what={"S".to_string()} route={Route::FindShoulder}  /> }
        }
        Route::FindKnee => {
            html! { <FindKnee what={"K".to_string()} route={Route::FindKnee}  /> }
        }
        Route::FindFoot => {
            html! { <FindFoot what={"F".to_string()} route={Route::FindFoot}  /> }
        }
        Route::Swagger => {
            html! { <Swagger  /> }
        }
        Route::SystemLogsBatch => {
            html! { <LogBatch  /> }
        }
        Route::SystemManual => {
            html! { <SystemManual /> }
        }
        Route::RateBtDate => {
            html! { <RateBtDatePage /> }
        }
    }
}
