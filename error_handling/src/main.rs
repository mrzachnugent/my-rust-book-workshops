use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};

fn main() {
    // panic!("crash and burn")

    {
        let _v = vec![1, 2, 3];

        // v[99];
        // To get backtrace run: `RUST_BACKTRACE=1 cargo run`
    }

    {
        let f = File::open("hello.txt");

        let _f = match f {
            Ok(file) => file,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!("Problem creating file: {:?}", e),
                },
                other_error => panic!("Problem opening the file: {:?}", other_error),
            },
        };

        println!("{}", fs::read_to_string("hello.txt").expect("msg: &str"))
    }

    {
        let _f = File::open("hello.txt").unwrap_or_else(|error| {
            if error.kind() == ErrorKind::NotFound {
                File::create("hello.txt").unwrap_or_else(|error| {
                    panic!("Problem creating the file: {:?}", error);
                })
            } else {
                panic!("Problem opening the file: {:?}", error);
            }
        });
    }

    {
        let _f = File::open("hello.txt").unwrap();
    }

    // Propagating Errors

    fn read_username_from_file() -> Result<String, io::Error> {
        let f = File::open("hello.txt");

        let mut f = match f {
            Ok(file) => file,
            Err(e) => return Err(e), // returns early as the `Result` if there's an error
        };

        let mut s = String::new();

        match f.read_to_string(&mut s) {
            Ok(_) => Ok(s), // puts content of File into `s`
            Err(e) => Err(e),
        }
    }

    {
        fn read_username_from_file() -> Result<String, io::Error> {
            let mut f = File::open("hello.txt")?; // `?` means if Result::Ok() continue with returned value from Ok() else return early
            let mut s = String::new();
            f.read_to_string(&mut s)?;
            Ok(s)
        }
    }

    // More refactoring
    {
        fn read_username_from_file() -> Result<String, io::Error> {
            let mut s = String::new();

            File::open("hello.txt")?.read_to_string(&mut s)?;

            Ok(s)
        }
    }

    {
        fn read_username_from_file() -> Result<String, io::Error> {
            fs::read_to_string("hello.txt")
        }
    }

    // Usually a good idea to handle your errors with Ressult
    // Exceptions: examples, prototype code, Tests, bad state, encoding user input.

    // Implementing our own validation
    {
        pub struct Guess {
            value: i32,
        }
        
        impl Guess {
            pub fn new(value: i32) -> Guess {
                if value < 1 || value > 100 {
                    panic!("Guess value must be between 1 and 100, got {}.", value);
                }
        
                Guess { value }
            }
        
            pub fn value(&self) -> i32 {
                self.value
            }
        }
        
    }
}
