use yew::{function_component, html, Html};

#[function_component]
pub fn SystemManual() -> Html {
    html! {
        <div>
            <h1>  {" 설명서 "} </h1>
            <ul class="manual">
                <li>
                    <h2> {"바운드"} </h2>
                </li>
            </ul>
        </div>

    }
}
