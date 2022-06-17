fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item
        }
    }

    largest
}

fn main() {
    // refactoring code to function to be re-usable
    {
        let nums = vec![34, 50, 25, 100, 65];

        println!("The largest number is.... : {}", largest_i32(&nums));

        let nums = vec![102, 34, 6000, 89, 54, 2, 43, 8];

        println!("The largest number is.... : {}", largest_i32(&nums))
    }

    // Both largest fn do the same thing but with different types
    {
        let number_list = vec![34, 50, 25, 100, 65];

        let result = largest_i32(&number_list);
        println!("The largest number is {}", result);

        let char_list = vec!['y', 'm', 'a', 'q'];

        let result = largest_char(&char_list);
        println!("The largest char is {}", result);
    }

    //  Using generics to refeactor into 1 fn (Almost works)
    // fn largest<T>(list: &[T]) -> T {
    //     let mut largest = list[0];

    //     for &item in list {
    //         if item > largest { // cannot compare Generic with `>`
    //             largest = item
    //         }
    //     }

    //     largest
    // }

    // Generics with Structs
    {
        struct Pointer<T> {
            x: T,
            y: T,
        }

        let interger = Pointer { x: 5, y: 5 };
        let float = Pointer { x: 5.0, y: 5.0 };

        // let cant_mix = Pointer { x: 5.0, y: 5}; // NO WORK, x and y must be same type
    }

    // Multiple generics in single struct
    {
        struct Point<T, U> {
            x: T,
            y: U,
        }

        let both_integer = Point { x: 5, y: 10 };
        let both_float = Point { x: 1.0, y: 4.0 };
        let integer_and_float = Point { x: 5, y: 4.0 };

        // Generics can also be used in `impl`
        impl<T, U> Point<T, U> {
            fn x(&self) -> &T {
                &self.x
            }
        }

        // We can also implement contraints!!
        // Only x and y of type f32
        impl Point<f32, f32> {
            fn distance_from_origin(&self) -> f32 {
                (self.x.powi(2) + self.y.powi(2)).sqrt()
            }
        }

        println!("p.x = {}", integer_and_float.x());
        println!("p.x = {}", integer_and_float.x);
        println!("{}", both_float.distance_from_origin())
    }

    // Generic type parameters in a struct definition aren’t always the same as those you use in that
    // same struct’s method signatures.

    {
        struct Point<X1, Y1> {
            x: X1,
            y: Y1,
        }

        impl<X1, Y1> Point<X1, Y1> {
            fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
                Point {
                    x: self.x,
                    y: other.y,
                }
            }
        }

        let p1 = Point { x: 5, y: 10.4 }; // X1: i32, Y1: f64
        let p2 = Point { x: "Hello", y: 'c' }; // X1: &str, Y1: char

        let p3 = p1.mixup(p2);
        // p1
        // X1: i32 => X1: i32
        // Y1: f64 => Y1: char
        // from X1: i32, Y1: f64 to X1: i32, Y1: char

        println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
    }

    // Generics don't slow you program down
    // because of Rust's behind the scenes MONOMORPHIZATION, generics are extremely efficiient at runtime

    // TRAITS
    // We can use trait bounds to specify that a generic type can be any type that has certain behavior.
    // Traits are similar to INTERFACES in other languages (wiith differences)

    // Defining a Trait
    // Similar to an interface, it is named `Summary`, has a method called `summarize` and returns a String
    // No method implementations.
    {
        pub trait Summary {
            fn summarize(&self) -> String;
        }

        pub struct NewsArticle {
            pub headline: String,
            pub location: String,
            pub author: String,
            pub content: String,
        }

        impl Summary for NewsArticle {
            fn summarize(&self) -> String {
                format!("{}, by {} ({})", self.headline, self.author, self.location)
            }
        }

        pub struct Tweet {
            pub username: String,
            pub content: String,
            pub reply: bool,
            pub retweet: bool,
        }

        impl Summary for Tweet {
            fn summarize(&self) -> String {
                format!("{}: {}", self.username, self.content)
            }
        }

        let tweet = Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        };

        println!("1 new tweet: {}", tweet.summarize());

        // We can implement a trait on a type only if at least one of the trait or the type is local to our crate.
        // Local: Summary, and Tweet
        // External: Display, Vec<T>

        // At Least one is local:
        // GOOD: We can implement the external Display TRAIT on our local Tweet TYPE
        // GOOD: We can implement the local TRAIT Summary on the external TYPE Vec<T>

        // CANNOT WORK: We cannot implement the external TRAIT Display on the external TYPE Vec<T>

        // This prevents other people’s code can’t break your code and vice versa.
    }

    // DEFAULT Implementations
    {
        pub trait Summary {
            fn summarize(&self) -> String {
                String::from("(Read more...)")
            }
        }

        pub struct NewsArticle {
            pub headline: String,
            pub location: String,
            pub author: String,
            pub content: String,
        }

        impl Summary for NewsArticle {} // implements default method

        let article = NewsArticle {
            headline: String::from("Penguins win the Stanley Cup Championship!"),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
            ),
        };

        println!("New article available! {}", article.summarize());
    }

    pub trait Summary {
        fn summarize_author(&self) -> String; // implementation needs to provide it's own fn

        fn summarize(&self) -> String {
            // Default and can be overriden
            format!("Read more from {}...", self.summarize_author())
        }
    }
    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    impl Summary for Tweet {
        fn summarize_author(&self) -> String {
            format!("@{}", self.username)
        }
        // fn summarize(&self) -> String {
        //     format!("Learn more from {}...", self.summarize_author())
        // } // can override default
    }
    {
        let tweet = Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        };

        println!("1 new tweet: {}", tweet.summarize());

        // Traits as Parameters
        // A parameter can implements the trait (allows for multiple types)
        // Here is the pretty version
        pub fn notify(item: &impl Summary) {
            println!("Breaking news! {}", item.summarize())
        }

        // Here is the actual syntax:
        // Trait Bound Syntax:
        pub fn notify_real<T: Summary>(item: &T) {
            // T is a generic that implements the Summary Trait
            println!("Breaking news! {}", item.summarize())
        }

        // With more parameters, the trait bound might be easier to read
        pub fn notify_m(item1: &impl Summary, item2: &impl Summary) {}

        pub fn notify_realm<T: Summary>(item1: &T, item2: &T) {}
    }

    // Specifying multiple traits
    {

        //  pub fn notify(item: &(impl Summary + Display)) {}

        // pub fn notify<T: Summary + Display>(item: &T) {}
    }

    use std::fmt::{Debug, Display};
    // Trait bound with WHERE clauses
    {
        // hard to read
        fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
            4
        }

        // `where` clause
        fn some_other_function<T, U>(t: &T, u: &U) -> i32
        where
            T: Display + Clone,
            U: Clone + Debug,
        {
            5
        }
    }

    // returning types that return traits
    // The function can return any item that implements Summary
    // BUT it CANNOT return different types that implements Summary
    // Examples:
    // 1. It CAN return a Tweet that implements Summary
    // 2. It CAN return a NewsArticle that implements Summary
    // 3. It CANNOT return either a Tweet or a NewsArticle
    {
        fn returns_summarizable() -> impl Summary {
            Tweet {
                username: String::from("horse_ebooks"),
                content: String::from("of course, as you probably already know, people"),
                reply: false,
                retweet: false,
            }
        }
    }

    // Fixing the `largest` function with Traits
    {
        fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
            // T must be orderable so we can use the `>` symbol
            // T must also be copyable (have a known size on the stack)
            // Alternitively we could ssay that T has the trait of Clone then call `.clone()` after `list[0]`
            let mut largest = list[0];

            for &item in list {
                if item > largest {
                    largest = item
                }
            }
            largest
        }

        let number_list = vec![34, 50, 25, 100, 65];

        let result = largest(&number_list);
        println!("The largest number is {}", result);

        let char_list = vec!['y', 'm', 'a', 'q'];

        let result = largest(&char_list);
        println!("The largest char is {}", result);
    }

    // Using Traits to conditiionnaly implement methods
    {
        struct Pair<T> {
            x: T,
            y: T,
        }

        // Method is implemented for EVERY Pair item
        impl<T> Pair<T> {
            fn new(x: T, y: T) -> Self {
                Self { x, y }
            }
        }

        // Method is ONLY implemented for Pairs that implement Display + PartialOrd traits
        impl<T: Display + PartialOrd> Pair<T> {
            fn cmp_display(&self) {
                if self.x >= self.y {
                    println!("The largest member is x = {}", self.x);
                } else {
                    println!("The largest member is y = {}", self.y);
                }
            }
        }
    }

    // Implementing a trait for a generic?!
    // We can conditionally implement methods for T based on the traits they have
    {
        // Basically, we want to implement a method on any type that implements the Display trait
        // Since ToString and Display are external, we cannot implement but it looks somehting like this:
        // impl<T: Display> ToString for T {}

        let s = 3.to_string();

        // AKA: Blanket implementationss
        // Traits vs Trait Bounds:
        // Trait: fn do_some(item: &impl Summary) {}
        //                         -------------
        // Trait Bonds: fn do_some<T: Summary> (item: &T) {}
        //                        ------------        --
    }

    // Validating references with lifetimes
    // Prevent dangling references with lifetimes
    {
        // let r;

        // {
        //     let x = 5;
        //     r = &x; // `x` does not live long enough
        //     // at the next line, x is out of scope, the value is removed from the stack to the reference no longer references `x`
        // }

        // println!("r: {}", r)

        // Let's remove the dangling reference
        let r;

        let x = 5;

        r = &x;

        println!("r: {}", r)
    }

    {
        let string1 = String::from("abcd");
        let string2 = "xyz";

        let result = longest(string1.as_str(), string2);
        println!("The longest string is {}", result);

        fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
            // the generic 'a lifetime tells the function that each associated parameter must live at least as long as each other's lifetime
            // This means we do not change the lifetimes of the parameters
            if x.len() > y.len() {
                x
            } else {
                y
            }
        }
    }

    // Lifetime annotations for Structs
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }
    {
        let i;
        let novel = String::from("Call me Ishmael. Some years ago...");
        {
            let first_sentence = novel.split('.').next().expect("Could not find a '.' (dot)");
            i = ImportantExcerpt {
                part: first_sentence,
            };
        }

        println!("{}", i.part) // Works because `novel` the original string lives long enough
                               // `first_sentence` is the string slice of novel
                               // storing that string slice of novel is still valid in i.part
    }

    // Don't always need to provide lifetime annotations when passing references that are "obvious"
    // Ex: fn first_word(s: &str) -> &str {} Can only return a ref to the same original string of the ref that was passed

    // Lifetime Annotations in method definitions
    {
        impl<'a> ImportantExcerpt<'a> {
            fn announce_and_return_part(&self, announcement: &str) -> &str {
                println!("Attention please: {}", announcement);
                self.part
            }
        }
    }

    // STATIC lifetime
    {
        let s : &'static str = "I have a static lifetime. I am stored in the program's biinary, which is always available.";

        // Before using `&'static` as the lifetime for a reference,
        // think about whether the reference you have actually lives the entire lifetime of your program or not,
        // and whether you want it to.
    }

    // Generic Type Parameters, Trait Bounds, and Lifetimes Together
    {
        fn longest_with_an_announcement<'a, T>(
            x: &'a str, // lifetime of 'a
            y: &'a str, // lifetime of 'a
            ann: T,     // has Display trait
        ) -> &'a str
        where
            T: Display, // returns str ref with lifetime of 'a
        {
            println!("Announcement! {}", ann);
            if x.len() > y.len() {
                x
            } else {
                y
            }
        }
    }
}
