// moved from lib.rs

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
