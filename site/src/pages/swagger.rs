use gloo::events::EventListener;
use gloo_utils::window;

use yew::{html::Scope, prelude::*};

use crate::request::addr::URL_SWAGGER;

pub enum Msg {
    //https://dev.to/fayaz/making-a-navigation-drawer-sliding-sidebar-with-tailwindcss-blueprint-581l
    Resize,
}

pub struct Swagger {
    width: f64,
    height: f64,
    _listener_resize: EventListener,
}
impl Component for Swagger {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let width: f64 = window().inner_width().unwrap().as_f64().unwrap();
        let height: f64 = window().inner_height().unwrap().as_f64().unwrap();
        //info!("create={},{}", width, height);
        let link: Scope<Swagger> = ctx.link().clone();
        let _listener_resize: EventListener =
            EventListener::new(&gloo::utils::window(), "resize", move |_event| {
                link.send_message(Msg::Resize);
            });

        Self {
            width,
            height,
            _listener_resize,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Resize => {
                let width: f64 = window().inner_width().unwrap().as_f64().unwrap();
                let height: f64 = window().inner_height().unwrap().as_f64().unwrap();
                //info!("{},{}", width, height);
                self.width = width;
                self.height = height;
            }
        }
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let url = format!("{}{}", dotenvy_macro::dotenv!("BACKEND_ADDR"), URL_SWAGGER);

        html! {
            <div class="overflow-x-hidden">
                <iframe
                    width="100%"
                    height={self.height.to_string()}
                    src={url}>
                </iframe>
            </div>
        }
    }
}
