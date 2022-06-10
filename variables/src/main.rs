const ONE_HOUR_IN_SECONDS: i32 = 60 * 60;

fn main() {
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
    )
}
