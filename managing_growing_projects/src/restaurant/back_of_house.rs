use super::front_of_house::serving;

pub enum Appetizer {
    Soup,
    Salad,
}

pub struct Breakfast {
    pub toast: String,
    seasonal_fruit: String,
}

impl Breakfast {
    pub fn summer(toast: &str) -> Breakfast {
        Breakfast {
            toast: String::from(toast),
            seasonal_fruit: String::from("peaches"),
        }
    }
}

fn fix_incorrect_order() {
    // root is back_of_house
    cook_order();
    // enter ../
    serving::server_order();
}

fn cook_order() {}
