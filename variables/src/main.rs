const ONE_HOUR_IN_SECONDS: i32 = 60 * 60;

fn main() {
    // 3.1
    // You must use `mut` to re-assign (bind) a new value to the variable
    let mut x = 5;
    println!("The value of x is {}.", x);
    x = 6;
    println!("The new value of x is {}.", x);

    // Constants can be declared outside the main scope (not variables)
    println!("There are {} seconds in an hour", ONE_HOUR_IN_SECONDS);

    // this `y` is always 5
    let y = 5;
    println!("The original value of y is {}.", y);

    // this `y` shadows the one above it, but it does not modify the original one
    let y = y + 1;
    println!("The shadowed value of y is {}.", y);

    // create inner scope (no need for new function, looping, or conditional statements)
    {
        // this `y` shadows the shadowed y. It does not modify the 2 previous "y"s.
        // It will use the value of the shadowed y to multiply by 2
        let y = y * 2;
        println!("The value of y in the inner scope is {}.", y)
    }

    println!(
        "The value of y is the first shadowed value (6). Here is y: {}.",
        y
    );

    let spaces = "   ";
    // Shadowing creates a new variable with same name AND ALLOWS TO ASSIGN (BIND) NEW TYPE
    // examples: from String to number type (usize)
    let spaces = spaces.len();

    // cannot re-assign mutable variable to different type.

    // cannot directly print line spaces must be string literal => cannot: println!(spaces);
    println!("{}", spaces);

    // 3.2
    // SCALAR TYPES (represent single value):

    // Integers:
    // Signed (given range from negative to positive) vs unsigned (same range as signed but only positive, starts at 0)
    // 8-bit	i8	u8
    // 16-bit	i16	u16
    // 32-bit	i32	u32
    // 64-bit	i64	u64
    // 128-bit	i128	u128
    // arch     isize   usize => depends on system, if 64-bit architecture or 32-bit

    let neg: i8 = -1;
    println!("signed i8 (8-bit) interger example: {}.", neg);

    // Floating point numbers (numbers with decimal points):
    // f32
    // f64 // more precise

    // addition
    let sum = 5 + 10; // default i32

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30; // default i32

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0 since default is i32

    // remainder
    let remainder = 43 % 5; // default i32

    println!(
        "sum {},difference {},product {},quotient {},floored {},remainder {}",
        sum, difference, product, quotient, floored, remainder
    );

    // Booleans: true, false

    let is_cool: bool = false;
    println!("Is cool? {}", is_cool);

    // characters (single quote, single character):
    // Can represent a lot more than just ASCII
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    println!("c {},z {},heart_eyed_cat {}", c, z, heart_eyed_cat);

    // COMPOUND TYPES
    // Tuples
    // Groups multiple types. Has a given length that cannot change.

    let tup: (u16, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;

    println!(
        "The deconstructed value (x, y,and z) from the tup are respectfully {}, {}, {}.",
        x, y, z
    );
    println!("Here is the first (0) was of the tuple {}", tup.0);

    let one = tup.2;

    println!("Here is the number one from the tuple: {}", one);

    let _unit: () = (); // tuple without any values is called a unit type

    // Arrays
    // Each item in this group must be of the same type and remains same size
    // data is allocated on the STACK rather than the HEAP
    // The vector type (from std lib) allows a flexible collection length

    // Type: [element type, length of array]
    let a: [u8; 6] = [1, 2, 3, 4, 5, 6];

    // initialize array with same value and given length
    let _b = [3; 5]; // [3,3,3,3,3]

    const CHOSEN_INDEX: usize = 3;
    println!(
        "Here is the element with an index of {}: {}",
        CHOSEN_INDEX, a[CHOSEN_INDEX]
    );

    println!("Please select the index of the array.");

    // create mutable empty string
    let mut index = String::new();

    std::io::stdin()
        // allow user to enter index
        .read_line(&mut index)
        // if error, show error from this message
        .expect("Failed to read item from index");

    // shadow initial index variable
    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );
}
