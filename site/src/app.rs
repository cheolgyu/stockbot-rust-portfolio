use crate::app::nav_state::{NavState, BP};
use crate::components::layout::{navbar::NavBar, side_nav::SideNav};
use crate::route::*;

use gloo::events::EventListener;
use gloo_console::console;
use gloo_utils::window;
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::HtmlElement;
use yew::prelude::*;
use yew_router::prelude::*;

pub enum Msg {
    Scroll(i32),
    ScrollTop,
    NavStateToggleMenu(bool),
    NavStateOpen(bool),
    WindowResize,
}

pub struct App {
    scroll_y: i32,
    nav_state: NavState,
    init_fab: bool,
    listener_resize: EventListener, //eeeeeeeelistener_location: LocationHandle,
}
impl App {
    fn scroll_top() {
        let document = window().document().expect("expecting a document on window");
        if let Some(element) = document.get_element_by_id("root") {
            let html_element: HtmlElement = element.dyn_into::<HtmlElement>().unwrap();
            html_element.set_scroll_top(0);
        } else {
            web_sys::console::log_1(&"해당 ID를 가진 요소를 찾을 수 없습니다.".into());
        }
    }
    fn event_listener_btn_fab(ctx: &Context<Self>) {
        let link = ctx.link().clone();
        let document = window().document().expect("expecting a document on window");
        if let Some(element) = document.get_element_by_id("root") {
            // 찾은 요소를 HtmlElement로 변환 (필요한 경우)
            let html_element: HtmlElement = element.dyn_into::<HtmlElement>().unwrap();

            let el2 = html_element.clone();
            let on_scroll_callback = Closure::<dyn FnMut(Event)>::new(move |_event: Event| {
                let scroll_top = el2.scroll_top();
                //console!(scroll_top.to_string());
                // console!(format!("scroll_y={}", scroll_y));
                link.send_message(Msg::Scroll(scroll_top));
            });

            html_element
                .add_event_listener_with_callback(
                    "scroll",
                    on_scroll_callback.as_ref().unchecked_ref(),
                )
                .unwrap();

            on_scroll_callback.forget();
        } else {
            web_sys::console::log_1(&"해당 ID를 가진 요소를 찾을 수 없습니다.".into());
        }
    }

    fn event_listener_resize(ctx: &Context<Self>) -> EventListener {
        let link: html::Scope<App> = ctx.link().clone();
        let listener_resize: EventListener =
            EventListener::new(&gloo::utils::window(), "resize", move |_event| {
                link.send_message(Msg::WindowResize);
            });
        listener_resize
    }
}
impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let mut nav_state = NavState::default();
        nav_state.resize();

        Self {
            scroll_y: 0,
            nav_state: nav_state,
            init_fab: false, // listener_location,
            listener_resize: App::event_listener_resize(ctx),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Scroll(y) => {
                //console!(y.to_string());
                self.scroll_y = y;
                self.init_fab = true
            }
            Msg::ScrollTop => {
                App::scroll_top();
            }
            Msg::NavStateToggleMenu(v) => {
                self.nav_state.toggle_nav();
            }
            Msg::WindowResize => self.nav_state.resize(),
            Msg::NavStateOpen(v) => self.nav_state.close(v),
        }
        true
    }

    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {}

    fn view(&self, ctx: &Context<Self>) -> Html {
        if !self.init_fab {
            App::event_listener_btn_fab(ctx);
        }
        let on_clicked = ctx.link().callback(|e| self::Msg::ScrollTop);
        let on_event_nav_state_toggle_menu = ctx.link().callback(self::Msg::NavStateToggleMenu);
        let on_event_nav_state_close = ctx.link().callback(self::Msg::NavStateOpen);

        let cls_app_open = self.nav_state.cls_app_open();
        let cls_app_open_app_page_div = self.nav_state.cls_app_open_app_page_div();
        let show_fab = self.scroll_y != 0;
        html! {
           <BrowserRouter>
                <div  class="app"   >
                    <div class="app_navbar">
                        <NavBar state={self.nav_state.clone()} cb={on_event_nav_state_toggle_menu} ></NavBar>
                    </div>

                   <div class="app_side_with_page">


                       <div class={cls_app_open}>
                            <div class=" cls_app_open_lg_sidenav ">
                                <SideNav state={self.nav_state.clone()} cb={on_event_nav_state_close} ></SideNav>
                            </div>
                       </div>

                        <div class={cls_app_open_app_page_div} id="root">
                                <div class="app_content">
                                    <Switch<Route> render={switch} />
                                </div>
                                if show_fab {
                                    <button class="fab" onclick={on_clicked }  >{"↑"}</button>
                                }
                                <div class="app_footer"></div>
                        </div>

                   </div>

               </div>
           </BrowserRouter>
        }
    }
}

pub mod nav_state {
    use gloo_utils::window;
    use serde::{Deserialize, Serialize};
    use yew::Properties;

    const BREAKPOINT_SM: f64 = 640.0;
    const BREAKPOINT_LG: f64 = 1024.0;

    #[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Properties)]
    pub struct NavState {
        pub open_nav: bool,
        pub screen: BP,
    }

    impl NavState {
        pub fn cls_side_open(&self) -> String {
            if self.screen == BP::LG {
                String::from("side_nav  ")
            } else {
                if self.open_nav {
                    String::from("side_nav_md  ")
                } else {
                    String::from(" side_nav_close")
                }
            }
        }

        pub fn cls_app_open(&self) -> String {
            if self.screen == BP::LG {
                String::from("cls_app_open_lg  ")
            } else {
                if self.open_nav {
                    String::from(" side_nav side_nav_open ")
                } else {
                    String::from(" hidden")
                }
            }
        }

        pub fn cls_app_open_app_page_div(&self) -> String {
            if self.screen == BP::LG {
                String::from("app_content_div  ")
            } else {
                if self.open_nav {
                    String::from(" hidden ")
                } else {
                    String::from(" app_content_div")
                }
            }
        }

        pub fn default() -> Self {
            Self {
                open_nav: false,
                screen: BP::SM,
            }
        }
        pub fn toggle_nav(&mut self) {
            self.open_nav = !self.open_nav
        }

        pub fn init_nav(&mut self) {
            self.open_nav = false;
            self.resize();
        }

        pub fn close(&mut self, v: bool) {
            self.open_nav = v;
        }

        pub fn resize(&mut self) {
            let width: f64 = window().inner_width().unwrap().as_f64().unwrap();
            if width > BREAKPOINT_LG {
                self.screen = BP::LG;
                self.open_nav = true;
            } else if BREAKPOINT_SM < width && width < BREAKPOINT_LG {
                self.screen = BP::MD;
                self.open_nav = false;
            } else {
                self.screen = BP::SM;
                self.open_nav = false;
            }
        }
    }

    #[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
    pub enum BP {
        SM,
        MD,
        LG,
    }
}
