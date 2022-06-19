//! # My Cargo and crates.io
//!
//! `cargo_and_crates_io` is a collection of utilities to make performing certain
//! calculations more convenient.
//! 
//! # Art
//!
//! A library for modeling artistic concepts.

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = cargo_and_crates_io::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
/// 
/// 
/// 


// To remove the internal organization from the public API, we can modify
pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub fn add_one(x: i32) -> i32 {
    x + 1
}



pub mod kinds {
    /// The primary colors according to the RYB color model.
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model.
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    /// Combines two primary colors in equal amounts to create
    /// a secondary color.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        // --snip--
        unimplemented!();
    }
}

