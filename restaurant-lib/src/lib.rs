#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

// Either of `lib.rs` or `main.rs` forms a module named crate at the root of the crate’s module structure.
// This module structure is known as the module tree.

// FOLLOWING MODULE TREE:
// crate
//  └── front_of_the_house
//      ├── hosting
//      │   ├── add_to_waitlist
//      │   └── seat_at_table
//      └── serving
//          ├── take_order
//          ├── serve_order
//          └── take_payment

// hosting nests inside of front_of_the_house (child)
// hosting and serving as siblings to each other

// Just like directories in a filesystem, you use modules to organize your code.
// And just like files in a directory, we need a way to find our modules.

// The following modules are private (to thier respective scopes)
mod front_of_the_house {
    // inline module declaration
    pub mod hosting {
        // inline module declaration in an inline module declaration
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        // anothing inline module in the front_of_the_house inline module declaration
        fn take_order() {}

        fn serve_orders() {}

        fn take_payment() {}
    }
}

// Paths for Referring to an Item in the Module Tree

// 1. Absolute path: starts from the crate root by using the crate name or a litteral `crate`
//    -------------

// 2. Relative path: starts from the current module and uses `self`, `super`, or an identifier in the current module.
//    -------------

// Both paths are followed by one or more identifiers seperated by `::`

// This function is declared in the crate root
pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_the_house::hosting::add_to_waitlist();
    // Absolute paths are easier to maintain
    // Ex: if we move this (eat_at_restaurant) function to another module, the absolute path to add_to_waitlist will still be valid

    // Relative path
    front_of_the_house::hosting::add_to_waitlist()
    // the hosting module needs to be public
    // AND the function (add_to_waitlist) needs to be public
}

// Modules are great for organizing but they are also PRIVACY BOUNDRIES

// Since eat_at_restaurant and front_of_the_house are declared in the same module,
// front_of_the_house does not need to be public for eat_at_restaurant to have access to it

// Starting relative paths with SUPER
// `super` is just like using `..` in relative directory paths

fn deliver_order() {}

mod back_of_the_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
        // deliver_order() // NO WORK
    }
    fn cook_order() {}

    // Making a Struct PUBLIC
    // Must explicitly make fields pub

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

    // Making an Enum PUBLIC
    // Just need to make enum `pub` to have all variantss public

    pub enum Appetizer {
        Soup,
        Salad,
    }
}

fn eat_breakfast_at_restaurant() {
    // make breakfast with Rye toast
    let mut meal = back_of_the_house::Breakfast::summer("Rye");

    // change your mind about toast kind
    meal.toast = String::from("Wheat");

    let order1 = back_of_the_house::Appetizer::Soup;
    let order2 = back_of_the_house::Appetizer::Salad;

    println!("I'd like {} toast please", meal.toast)
}

use crate::front_of_the_house::hosting;
fn eat_at_restaurant_with_use() {
    hosting::add_to_waitlist();
}

// Idiomatic path specification
// use crate::front_of_the_house::hosting::add_to_waitlist; // NOT SUGGESTED FOR FUNCTION
// Specifying the parent module when calling the function makes it clear that the function isn’t locally defined 
// while still minimizing repetition of the full path.

// Idiomatic structs, enumss, and non-function items is a convention thatt has EMERGED.
// People are just use to writing and reading them like that


// The exception: Cannot bring two `Result` types with same name
// use std::fmt::Result;
// use std::io::Result;

// This is how you use 2 results:

// 1.
use std::fmt;
use std::io;

fn function1() -> fmt::Result {
    // --snip--
    println!("fmt::Result");
    fmt::Result::Ok(())
}

fn function2() -> io::Result<()> {
    // --snip--
    println!("io::Result");
    io::Result::Ok(())
}

// 2.
use std::fmt::Result;
use std::io::Result as IoResult; // renamed `std::io::Result` to `IoResult`


// When bringing a name into scope (similar to importing in JS),
// that name become private in that scope

// Similar to exporting in JS, you can export what you are importing
// In Rust, you can combine `pub` and `use` -> `pub use`

pub use crate::front_of_the_house::hosting::add_to_waitlist;

// Using NESTED PATHS:

use std::{cmp::Ordering, io::copy};

// much better than writing:
// use std::cmp::Ordering;
// use std::io::copy

use std::io::Bytes::{self/*, other_imports */}; // self => become Bytes

// The Glob Operator:
// Brings all public items defined in a path into scope
use std::collections::*;

mod front_of_house;

pub use crate::front_of_house::hostingger;

pub fn eat_at_some_restaurant() {
    hostingger::add_to_waitlist();
}
