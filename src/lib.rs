// modules start with "mod" derf
// and can have nested modules


mod front_of_house {
    // child module of front_of_house
    mod hosting {
        fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    // child module of front_of_house
    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }

}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
