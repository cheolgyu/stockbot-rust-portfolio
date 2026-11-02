use gloo::events::EventListener;
use gloo_console::log;
use rm_model::Bound;
use wasm_bindgen::{JsCast, JsValue, UnwrapThrowExt};
use yew::html::Scope;
use yew::prelude::*;
use yew::prelude::*;
use yew_router::prelude::*;
#[function_component(App)]
fn app() -> Html {
    use_effect(move || {
        // Attach a keydown event listener to the document.
        let document = gloo::utils::window();
        let listener = EventListener::new(&document, "resize", |event| {
            //let event = event.dyn_ref::<web_sys::KeyboardEvent>().unwrap_throw();
            // In this example, we just log the event, but obviously this is where your code goes.
            gloo_console::log!(event);
        });

        // Called when the component is unmounted.  The closure has to hold on to `listener`, because if it gets
        // dropped, `gloo` detaches it from the DOM. So it's important to do _something_, even if it's just dropping it.
        || drop(listener)
    });

    html! {
        <h1>{ "Hello World2 " }</h1>
    }
}

pub enum Msg {
    Hello,
}
pub struct AppS {
    //listener: LocationHandle,
}
impl Component for AppS {
    type Message = Msg;

    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let link = ctx.link().clone();

        // let listener = ctx
        //     .link()
        //     .add_location_listener(link.callback(move |_| Msg::Hello))
        //     .unwrap();

        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let item = Bound { code_id: 1111111 };
        html! {
            <>
            <h1>{ "Hello World1" }</h1>
            <h1>{ format!("{:?}",item) }</h1>
            </>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Hello => {
                gloo_console::log!("hello");
            }
        }
        true
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        true
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {}

    fn prepare_state(&self) -> Option<String> {
        None
    }

    fn destroy(&mut self, ctx: &Context<Self>) {}
}

impl AppS {
    fn window_event(&self, ctx: &Context<Self>) {
        let document: web_sys::Window = gloo::utils::window();
        let listener: EventListener = EventListener::new(&document, "resize", |event| {
            //let event = event.dyn_ref::<web_sys::KeyboardEvent>().unwrap_throw();
            // In this example, we just log the event, but obviously this is where your code goes.
            gloo_console::log!(event);
        });
    }
}

fn main() {
    yew::Renderer::<AppS>::new().render();
}
