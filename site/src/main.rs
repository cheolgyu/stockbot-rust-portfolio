mod app;
mod components;
mod impl_param;
mod js;
mod name;
mod pages;
mod request;
mod route;
mod utils;

use app::App;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
#[cfg(test)]
mod tests {
    use batch_model::site::param::bound_list::Param;

    #[test]
    fn set_value_to_tuple() {
        let item = Param::default();
        println!("{:#?}", item);
        let encoded = serde_urlencoded::to_string(&item).unwrap();
        println!("encoded={:#?}", encoded);
        let decoded: Param = serde_urlencoded::from_str(&encoded).unwrap();
        println!("decoded={:#?}", decoded);
        let result = 4;
        assert_eq!(result, 4);
    }
}
