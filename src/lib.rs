use std::fmt::Result;
use std::io::Result as IoResult;
// use std::io;
// use std::io::Write;
use std::io::{self, Write};


fn function1() -> Result {
    Ok(())
}

fn function2() -> IoResult<()> {
    Ok(())
}

mod front_of_house;


// Here it equals use self::front_of_house::hosting;
use crate::front_of_house::hosting;
// pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // thanks to the use keyword
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}


fn serve_order() {}

mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }

    fn fix_incorrect_order() {
        cook_order();
        // super keyword
        // Here it equals crate::serve_order();
        super::serve_order();
    }

    fn cook_order() {}
}




