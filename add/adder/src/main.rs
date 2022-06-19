use add_one;
use add_two;

fn main() {
    let num = 10;
    println!(
        "Hello, world! {} plus one is {} or plus two is {}!",
        num,
        add_one::add_one(num),
        add_two::add_two(num)
    );
}
