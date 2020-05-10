// modules start with "mod" derf
// and can have nested modules

/* moved to front_of_house.rs
mod front_of_house {
    // child module of front_of_house
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    // child module of front_of_house
    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}
*/

mod front_of_house;
pub use crate::front_of_house::hosting;

// "Our preference is to specify absolute paths because it’s more likely to
// move code definitions and item calls independently of each other."
// ch07-03-paths-for-referring-to-an-item-in-the-module-tree

pub fn eat_at_restaurant() {
    // using absolute paths with the `crate` keyword
    //crate::front_of_house::hosting::add_to_waitlist();

    // front_of_house module was moved to it's own file
    // so we have to update the path
    hosting::add_to_waitlist();

    // using relative paths
    // because these modules are at the same level of the module/dir structure
    //front_of_house::hosting::add_to_waitlist();

    // front_of_house module was moved to it's own file
    // so we have to update the path
    hosting::add_to_waitlist();


    // order breakfast in the summer with rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // change your mind about the toast
    meal.toast = String::from("wheat");
    println!("I'd like {} toast please", meal.toast);
    // this works because the summer function is public
    // and that function constructs a public instance of Breakfast

    // this line won't compile because seasonal_fruit isn't public
    // error[E0616]: field `seasonal_fruit` of struct `back_of_house::Breakfast` is private
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        // works like `../`, up-one-level
        // in this case, access to `serve_order`, which lives one level up
        // from the back_of_house module
        super::serve_order();
    }

    fn cook_order() {}

    // structs can be public too and not all of the fields have to be
    // in this case, customers can choose the toast but no the fruit
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

    // public enum, all variants are public
    // note to self: variant vs variations?
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
