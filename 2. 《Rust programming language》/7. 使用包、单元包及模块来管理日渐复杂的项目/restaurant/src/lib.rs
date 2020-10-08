mod front_of_house {    // 同级不需要pub
    pub mod hosting {    // 子级模块需要pub
        pub fn add_to_waitlist() { }    // 子级条目需要pub
        fn add_to_table() { }
    }
    
    mod serving {
        fn take_order() { }
        fn serve_order() { }
        fn take_payment() { }
        mod back_of_house {
            fn fix_incorrect_order() {
                cook_order();
                super::serve_order();
            }
            fn cook_order() { }

            pub struct Breakfast {
                pub toast: String,
                seasonal_fruit: String,
            }
            impl Breakfast {
                pub fn summer(toast: &str) -> Breakfast {    // 公共关联函数，用于构造Breakfast实例
                    Breakfast {
                        toast: String::from(toast),
                        seasonal_fruit: String::from("peaches"),
                    }
                }
            }

            pub enum Appetizer {
                Soup,
                Salad,
            }
        }

        pub fn eat_at_restaurant2() {
            let mut meal = back_of_house::Breakfast::summer("Rye");
            meal.toast = String::from("Wheat");
            println!("I'd like {} toast please", meal.toast);

        // meal.seasonal_fruit = String::from("blueberries");    // 编译报错
        }

        pub fn eat_at_restaurant3() {
            let order1 = back_of_house::Appetizer::Soup;
            let order2 = back_of_house::Appetizer::Soup;
        }
    }
}

pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist();    // 绝对路径
    front_of_house::hosting::add_to_waitlist();    // 相对路径
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant4() {
    hosting::add_to_waitlist();
}