// use cargo_and_crates_io::kinds::PrimaryColor;
// use cargo_and_crates_io::utils::mix;
use cargo_and_crates_io::{PrimaryColor, mix};

fn main() {
    println!("This is where I can see what the dev experience iss like when using my package.");
    println!("I can also provide extra examples here I think.");
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    let _mix = mix(red, yellow);
}
