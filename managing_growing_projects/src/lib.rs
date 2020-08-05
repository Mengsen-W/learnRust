pub mod restaurant;
fn serve_order() {}

pub fn eat_at_restaurant() {
    // root with this file
    // Absolute path
    crate::restaurant::front_of_house::hosting::add_to_waitlist();

    // Relative path ./
    restaurant::front_of_house::hosting::add_to_waitlist();

    let mut meal = restaurant::back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    // meal.seasonal_fruit = String::from("blueberries");
    
    let order1 = crate::restaurant::back_of_house::Appetizer::Soup;
    let order2 = crate::restaurant::back_of_house::Appetizer::Salad;
    
}
