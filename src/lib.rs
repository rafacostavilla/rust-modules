mod front_of_house;

//Re-export
pub use crate::front_of_house::hosting;

mod back_of_house {
    
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }
    
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast{
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
    
    // If a enum is public, all its variants are then public
    pub enum Appetizer {
        Soup,
        Salad,
    }

    fn fix_incorrect_order(){
        cook_order();
        super::deliver_order();
    }
    // reference the outer shortcut
    fn cook_order() {super::hosting::add_to_waitlist();}
}

mod serving{
    fn take_order() {}

    fn serve_order() {}

    fn take_payment() {}
}

fn deliver_order() { hosting::add_to_waitlist()}

pub fn eat_at_restaurant() {
    //Absolute path
    crate::front_of_house::hosting::add_to_waitlist();
    
    //Relative path
    front_of_house::hosting::add_to_waitlist();

    hosting::add_to_waitlist();

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

pub fn eat_breakfast_at_restaurant (){
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // meal.seasonal_fruit = String::from("blueberries");
}