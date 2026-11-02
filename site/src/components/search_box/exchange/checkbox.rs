use std::str::FromStr;

use batch_model::app_type::exchange_type::ExchangeType;
use gloo_utils::window;

use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct Checkbox {
    pub key: ExchangeType,
    pub text: String,
    pub checked: bool,
}

#[derive(Debug)]
pub struct CheckBoxExchange {
    base: Vec<Checkbox>,
}
#[derive(Clone, PartialEq, Properties)]
pub struct Properties {
    pub cb_exchange: Callback<Vec<ExchangeType>>,
    /// 기본선택값: KOSPI,KOSDAQ
    pub default_checked_key: Vec<ExchangeType>,
}
pub enum Msg {
    EventChange(InputEvent),
}

impl Component for CheckBoxExchange {
    type Message = Msg;
    type Properties = Properties;

    fn create(ctx: &Context<Self>) -> Self {
        //ctx.link().send_future(async { Msg::Fetch });
        let mut item = CheckBoxExchange::new();
        item.change_checked(ctx.props().default_checked_key.clone());

        item
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::EventChange(e) => {
                let el_attr_key = e
                    .target_dyn_into::<HtmlInputElement>()
                    .unwrap()
                    .get_attribute("attr_key")
                    .unwrap();
                let key = ExchangeType::from_str(&el_attr_key).unwrap();

                if self.required_1(key) {
                    self.update_value(key);
                    ctx.props().cb_exchange.emit(self.checked());
                } else {
                    window().alert_with_message("1개는 필수 입니다.").unwrap();
                }
            }
        }
        true
    }

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_event = ctx.link().callback(self::Msg::EventChange);
        let paramm_exist = ctx.props().default_checked_key.clone();

        let item = self
            .base
            .clone()
            .iter()
            .map(|c| {
                html! {
                    <div>
                        <label for={c.key.to_string()} class="cursor-pointer">
                            <input
                                class="checkbox_exchange checked:bg-blue-500"
                                type="checkbox"
                                name={c.key.to_string()}
                                id={c.key.to_string()}
                                oninput={on_event.clone()}
                                checked={c.checked} attr_key={c.key.to_string()} />
                            {&c.text}
                        </label>
                    </div>
                }
            })
            .collect::<Html>();

        html! {
            <fieldset class="">
                <legend>{"거래소"}</legend>
                {item}
            </fieldset>

        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {}

    fn prepare_state(&self) -> Option<String> {
        None
    }

    fn destroy(&mut self, _ctx: &Context<Self>) {}
}

impl CheckBoxExchange {
    pub fn new() -> Self {
        CheckBoxExchange {
            base: vec![
                Checkbox {
                    key: ExchangeType::KOSPI,
                    text: ExchangeType::KOSPI.text(),
                    checked: true,
                },
                Checkbox {
                    key: ExchangeType::KOSDAQ,
                    text: ExchangeType::KOSDAQ.text(),
                    checked: true,
                },
                Checkbox {
                    key: ExchangeType::KONEX,
                    text: ExchangeType::KONEX.text(),
                    checked: false,
                },
            ],
        }
    }

    // pub fn new_checked(v: Vec<ExchangeType>) -> Self {
    //     let item: Self = Self::new();
    //     let mut res: Vec<Checkbox> = item.base;

    //     for i in 0..res.len() {
    //         let item = res.0[i].key.to_string();
    //         let mut is_true = false;
    //         for j in 0..v.len() {
    //             if item == v[j] {
    //                 is_true = true;
    //             }
    //         }
    //         res.0[i].checked = is_true;
    //     }

    //     res
    // }

    pub fn change_checked(&mut self, inp: Vec<ExchangeType>) {
        if !inp.is_empty() {
            for c in self.base.iter_mut() {
                let mut c_checked = false;
                for i in inp.iter() {
                    if &c.key == i {
                        c_checked = true;
                        break;
                    }
                }
                c.checked = c_checked;
            }
        }
    }

    pub fn checked(&self) -> Vec<ExchangeType> {
        let mut res = vec![];
        let items = &self.base;
        for item in items {
            if item.checked {
                res.push(item.key);
            }
        }
        res
    }

    pub fn update_value(&mut self, key: ExchangeType) {
        let items = &mut self.base;
        for item in items {
            if key == item.key {
                item.checked = !item.checked;
                break;
            }
        }
    }

    pub fn required_1(&mut self, key: ExchangeType) -> bool {
        let mut sum_chked = 0;
        let items = &mut self.base;
        let mut inp: Checkbox = Checkbox::default();
        for item in items.iter() {
            if item.key == key {
                inp = item.clone();
            }
            if item.checked {
                sum_chked += 1;
            }
        }

        !(!!inp.checked && sum_chked == 1)
    }

    // pub fn query_format(list: Vec<String>) -> String {
    //     let mut v_exchange = "".to_string();
    //     if list.len() > 0 {
    //         v_exchange = list.get(0).unwrap().to_string();
    //         for x in 1..list.len() {
    //             let v = list.get(x).unwrap().to_string();
    //             v_exchange = v_exchange + "," + &v;
    //         }
    //     }

    //     v_exchange
    // }

    // pub fn checked_to_vec(&self) -> Vec<String> {
    //     let mut exchange_res: Vec<String> = vec![];

    //     for v in &self.0 {
    //         if v.checked {
    //             exchange_res.push(v.key.to_string())
    //         }
    //     }
    //     exchange_res
    // }

    // pub fn valid_str(v: String) -> Vec<String> {
    //     let exchange_vec: Vec<&str> = v.split(",").collect();
    //     let mut exchange_res: Vec<String> = vec![];
    //     for x in exchange_vec {
    //         if Key::from_str(x).is_ok() {
    //             exchange_res.push(x.to_string())
    //         }
    //     }
    //     exchange_res
    // }
}
