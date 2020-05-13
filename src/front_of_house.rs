// moved from lib.rs

// load hosting.rs
pub mod hosting;

// child module of front_of_house
mod serving {
    fn take_order() {}
    fn serve_order() {}
    fn take_payment() {}
}
