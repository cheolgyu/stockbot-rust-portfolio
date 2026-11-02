pub mod agg;
pub mod app_type;
pub mod batch_timer;
pub mod bound;
pub mod code;
pub mod country;
pub mod find;
pub mod format;
pub mod graph;
pub mod log;
pub mod news;
pub mod price;
pub mod price_chg;
pub mod rate_by_date;
#[cfg(feature = "site")]
pub mod site;
pub mod utils;
pub extern crate tb_row;
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

//https://github.com/dtolnay/request-for-implementation

struct Waffles;
#[cfg(test)]
mod tests {
    use serde::ser::SerializeTupleStruct;
    use serde::Serializer;

    use tb_row::TbRow;
    #[derive(TbRow, Default, Debug)]
    struct FrenchToast {
        #[order(2)]
        pub id: usize,
        #[order(0)]
        pub short_name: String,
        #[order(1)]
        pub price: i64,
    }

    #[test]
    fn set_value_to_tuple() {
        use serde_json::json;

        let mut i = FrenchToast::default();

        i.price = 123;
        i.short_name = "f_toast".to_string();
        i.id = 321;

        let j = json!(i);
        println!("result ={:#?}", j);

        // result =Array [
        //     String("f_toast"),
        //     Number(123),
        //     Number(321),
        // ]

        let result = 4;
        assert_eq!(result, 4);
    }

    // #[test]
    // #[cfg(feature = "site")]
    // fn it_works() {
    //     use crate::site::res::volume::view::{FrenchToast, Test2, Waffles};
    //     let t: Test2 = Test2 {
    //         code_id: 123,
    //         short_code: "zhem".to_string(),
    //         short_name: "이름".to_string(),
    //     };
    //     FrenchToast::helloworld();
    //     println!("result ={:#?}", FrenchToast::return_usize());
    //     println!("result ={:#?}", FrenchToast::return_string());
    //     println!("result ={:#?}", FrenchToast::return_vec_string());
    //     println!("result ={:#?}", FrenchToast::th_name() ());
    //     let cols = FrenchToast::th_name() ();
    //     println!("cols ={:#?}", cols);
    //     for i in cols {
    //         println!("i={}", i);
    //         if i == "code_id".to_string() {
    //             println!("code_idcode_idcode_idcode_idi={}", i);
    //         }
    //     }
    //     let result = 4;
    //     assert_eq!(result, 4);
    // }
}
