fn main() {
    let number = 3;

    if number < 5 && number > 7 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    if number != 0 {
        println!("number was something other than zero");
    }

    // shadowing because im lazy and copy pasted from the rust book
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = false;
    // similar to turneries, binding if else with implicit return to a (shadow) variable
    // if / else MUST HAVE SAME RETURN TYPE since variables must have a single type
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);

    let my_string = if condition { "I'm true" } else { "I'm false" };

    println!("my_string holds (is bound to): {}", my_string);

    // LOOPS: while, for, loop

    // loop {
    //     println!("INFINITE LOOP")
    // }

    loop {
        println!("This will loop until the keyword 'break' is excecuted.");
        // 'continue' would start the loop over without executing code under it.
        break;
    }

    let mut count = 0;

    'counting_up: loop {
        //'counting_look is a LABEL. Allows to break from this given loop within other loops
        println!("Count: {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);

    let mut counter = 0;

    let result = loop {
        // since loop returns something we can bind it to a variable
        counter += 1;

        if counter == 10 {
            break counter * 2; // breaks and returns counter times two
        }
    };

    println!("The result is {}", result);

    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {}", element);
    }

    // (num..num2) creates a range from num to num2
    // .rev() reverses the sequenced range of numbers
    for num in (1..4).rev() {
        println!("{}!!!!", num)
    }
}
