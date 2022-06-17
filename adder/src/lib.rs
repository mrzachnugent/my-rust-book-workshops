#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add_two(a: i32) -> i32 {
    internal_aadder(a, 2)
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name) // Changed to introduce bug
                               // String::from("Hello")
}

fn internal_aadder(a: i32, b: i32) -> i32 {
    a + b
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        // if value < 1 {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100.")
        }
        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*; // to access all of the tests modules parent items
    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }
    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert_ne!(smaller.can_hold(&larger), true);
    }

    #[test]
    fn greeting_contains_name() {
        let name = "Carol";
        assert!(
            greeting(name).contains(name),
            "Greeting did not contain name, value was {}",
            greeting(name)
        );
    }

    #[test]
    #[should_panic(expected = "Guess value must be between 1 and 100.")] // must panic with exact message
    fn greater_than_100() {
        Guess::new(200);
        // static method uses `::` notation, they do not have a &self arg
        // whereas items of type `Guess` can use the dot `.` notation for methods
    }

    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    // Since tests run in parallel make sure their independent from each other

    // If you need to run them consecutively, you can run the test with the command: `$ cargo test -- --test-threads=1`

    // println! are not shown in test, if you want to see them, run the following command:
    // `cargo test -- --show-output`

    // Testing by subset of test name
    // `cargo test it_works` will only test that one fn

    #[test]
    fn add_two_second() {
        assert!(add_two(5) == 7)
    }

    // `cargo test add`
    // runs all tests with add in the name of the test function
    // running 2 tests
    // test tests::it_adds_two ... ok
    // test tests::add_two_second ... ok

    // ignoring some test
    #[test]
    #[ignore]
    fn expensive_tesst() {
        // code that takes an hour to run
        // result is `ok` since it does not fail
    }

    // running only ignored tests
    // `cargo test -- --ignored`

    // Test Organization
    // UNIT vs INTERGRATION test

    // Unit test:Small and more focused, testing one module in isolation at a time, and can test private interfaces
    // ---------

    // Intergration test:Entirely external to your library and use your code in the same way any other external code would, 
    // ----------------- using only the public interface and potentially exercising multiple modules per test

    //  The Unit test convention is to create a module named tests in each file to contain the test functions and
    // to annotate the module with cfg(test).
    
    // `#[cfg(test)]` annotation tells run to only compile and run this code when you run `cargo build`

    // Integration tests do not use `#[cfg(test)]`. Since unit tests are in the same files as the code, it is necessary

    // INTERNAL (private) functions
    // Rust’s privacy rules do allow you to test private functions.
    #[test]
    fn internal() {
        assert_eq!(4, internal_aadder(2,2));
    }
}
