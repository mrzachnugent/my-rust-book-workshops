fn another_function(x: i32) {
    println!("Printing line from another function (not main).");
    println!("Here is the provided (i32 type) argument: {}!", x)
}

fn main() {
    println!("HEYYOOO, I'm in main");
    another_function(28);
    another_other_function();
    print_weight(210, "lbs");

    // statement
    let six = 6;

    // expression
    let twelve = {
        let double = 2;
        six * double // important! no semi colon here!
    };
    println!("twelve: {}", twelve);

    // expression within new scope
    // This example returns 4 but is not used or bound (binded) to anything
    {
        let x = 3;
        x + 1
    };

    let x = five();

    println!("five: {}", x);
    println!("1 plus one is: {}", plus_one(1))
}

fn another_other_function() {
    println!("HOWDY HO! I'm from another other function")
}

fn print_weight(weight_num: u32, unit: &str) {
    println!("Here is the enttered weight: {} {}.", weight_num, unit);
}

// "most functions return the last expression implicitly.""
fn five() -> i32 {
    5 // last expression without ";" is implicitly returned
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

// 3.4 comments. This is an example in itself.
// There are also documentation comments we'll see later in chap. 14
