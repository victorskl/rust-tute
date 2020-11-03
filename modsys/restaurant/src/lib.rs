// this source file is the default library crate by convention using file name `lib.rs`

// --- auto-generated by cargo new --lib restaurant

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

// --- start coding here

// nested modules
/*
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}
*/
// split front_of_house mod into separate file `front_of_house.fs`
mod front_of_house;

use crate::front_of_house::hosting as abs_hosting_path; // absolute path using 'crate'
use self::front_of_house::hosting; // relative path using 'self'
use crate::front_of_house::hosting::add_to_waitlist; // bring in all the way to the function
pub use crate::front_of_house::hosting as reexported_abs_hosting_path; // re-exporting using pub

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();
    // can be shorten using `use`
    hosting::add_to_waitlist();
    // or, all the way to fn
    add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // ---

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    // ---

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {
        println!("cook_order called");
    }

    // ---

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

    // ---

    pub enum Appetizer {
        // the default for enum variants (enum elements 👇) are to be public
        Soup,
        Salad,
    }
}