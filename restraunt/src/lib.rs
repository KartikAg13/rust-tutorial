//nested paths
use rand::{Rng, CryptoRng, ErrorKind::Transient};
use std::io::{self, Write};

//glob operator
use std::cmp::*;

mod front_of_house;

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        //super allows us to refer to the parent module (crate)
        super::serve_order();
    }

    fn cook_order() {}

    pub struct Breakfast {
        //even if the struct is public, by default the field are private
        pub toast:String,
        seasonal_fruit:String
    }

    impl Breakfast {
        pub fn summer(toast:&str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches")
            }
        }
    }

    pub enum Appetizer {
        //if an enum is public, all of its variants become public
        Soup,
        Salad
    }
}

//using pub makes sure that we can use this module in another file also
pub use self::front_of_house::serving as serve;

pub fn eat_at_restaurant() {
    let _secret_number = rand::thread_rng().gen_range(1, 101);

    //absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    //relative path
    front_of_house::hosting::add_to_waitlist();

    //use of 'use' and 'as'
    serve::take_order();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");

    let _order    = back_of_house::Appetizer::Soup;
}

fn serve_order() {}