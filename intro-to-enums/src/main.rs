fn main() {
    println!("WELCOME TO INTRO TO ENUMURATIONS!");
    {
        // Enumerations
        // Like typing for a value that can be one of the following in the enum.
        // Struct is closer to class with key/value pairs and Enums are closer to typing for a value that can have one of the following variants?
        // IP address can either be V4 or V6 (not both at the same time)

        #[derive(Debug)]
        enum IpAddressKind {
            V4, // what is the type of V4? => the type of IpAddressKind
            V6, // second variant of the IpAddressKind namespace
        }

        let four = IpAddressKind::V4;
        let six = IpAddressKind::V6;

        println!("{:?} or {:?}", four, six);

        fn route(_ip_kind: IpAddressKind) {
            println!("Called route fn with variant of type enum named IpAddresKind");
        }

        route(six);
    }

    // Structs and Enums work well together
    {
        #[derive(Debug)]
        enum IpAddressKind {
            V4, // what is the type of V4? => the type of IpAddressKind
            V6, // second variant of the IpAddressKind namespace
        }

        struct IpAddress {
            kind: IpAddressKind,
            address: String,
        }

        let home = IpAddress {
            kind: IpAddressKind::V6,
            address: String::from("127.0.0.1"),
        };

        let loopback = IpAddress {
            kind: IpAddressKind::V4,
            address: String::from("::1"),
        };
    }

    // Representing the same concept using just an enum is more concise

    {
        enum IpAddr {
            V4(String),
            V6(String),
        }

        let home = IpAddr::V4(String::from("127.0.0.1"));

        let loopback = IpAddr::V6(String::from("::1"));
    }

    // Each variant can have different types and amounts of associated data
    {
        enum IpAddr {
            V4(u8, u8, u8, u8),
            V6(String),
        }

        let home = IpAddr::V4(127, 0, 0, 1);

        let loopback = IpAddr::V6(String::from("::1"));
    }

    // There already exists a library for IP Address, it uses enums where the variats asssociate Structs
    {
        struct Ipv4Addr {
            // -- code snippet --
        }

        struct IpV6Addr {
            // -- code snippet --
        }

        enum IpAddr {
            V4(Ipv4Addr),
            V6(IpV6Addr),
        }

        // you can even have an enum associated to the enum variants
    }

    {
        #[derive(Debug)]
        enum Message {
            Quit,                       // no data associated to it
            Move { x: i32, y: i32 }, // named fields associated to this variant (similar to a struct)
            Write(String),           // includes a single String
            ChangeColor(i32, i32, i32), // includes 3 i32 values
        }

        // Message enum from above ^^ is similar to defining 4 structs
        // 1.
        struct QuitMessage; // unit struct

        // 2.
        struct MoveMessage {
            x: i32,
            y: i32,
        }

        //3.
        struct WriteMessage(String); //tuple struct

        // 4.
        struct ChangeColorMessage(i32, i32, i32); // tuple struct

        // Each struct has thier own type so if we wanted to create a function that accepted either of
        // these structs, it wouldn't be as easy to create as with an enum

        // enums can also implement functions
        impl Message {
            fn call(self: &Self) {
                // verbose way of writing: fn call(&self) {
                // method body would be defined here
                println!("{:?}", self) // only able to print line since message is derived for debugging
            }
        }

        let m = Message::Write(String::from("Hello from MARS"));
        m.call() // prints: Write("Hello from MARS")
    }

    // Option enums and other null values
    {
        // Option is an enum defined by the standard library (std)
        // Option: Could be something or could be nothing
        // Rust does not have Null as a value

        // here is the Option enum:
        enum Option<T> {
            // <T> is a generic we'll cover in Chapter 10
            None,
            Some(T), // can hold one piece of data of any type
        }
    }

    {
        // The Option<T> enum is so useful that it’s even included in the prelude.
        // You don’t need to bring it into scope explicitly.

        let some_number = Option::Some(5);
        let some_string = Some("a string (&str)"); // no need to bring Some from Option namespace

        let abcent_number: Option<i32> = None;
    }

    {
        let x: i8 = 5;
        let y: Option<i8> = Some(5);

        //let sum = x + y; // NO WORK cant i8 + Option<i8>. What is Option<i8> is null?
    }

    // The MATCH CONTROL FLOW CONSTRUCT
    {
        enum Coin {
            Penny,
            Nickle,
            Dime,
            Quarter,
        }

        fn value_in_cents(coin: Coin) -> u8 {
            match coin {
                Coin::Penny => {
                    println!("LUCKY PENNY!!!");
                    1
                }
                Coin::Nickle => 5,
                Coin::Dime => 10,
                Coin::Quarter => 25,
            }
        }

        let half_of_fifty = Coin::Quarter;

        let penny = Coin::Penny;

        println!("Half of 50 is: {}", value_in_cents(half_of_fifty));

        println!("Penny? {}", value_in_cents(penny));
    }

    {
        // CHANGING one of our enum variants to hold data inside it.

        #[derive(Debug)]
        enum UsState {
            Alabama,
            Alaska, // -- rest ---
        }

        enum Coin {
            Penny,
            Nickle,
            Dime,
            Quarter(UsState),
        }

        // Attempting to find all the quarters from the 50 different state,
        // We will say where the quarter if from if we get a quarter

        fn value_in_cents(coin: Coin) -> u8 {
            match coin {
                Coin::Penny => 1,
                Coin::Nickle => 5,
                Coin::Dime => 10,
                Coin::Quarter(state) => {
                    println!("State quarter from {:?}!", state); // state binds to the value of UsState::Alaska
                    25
                }
            }
        }

        value_in_cents(Coin::Quarter(UsState::Alabama));
    }

    // Matching with Option<T>
    {
        fn plus_one(x: Option<i32>) -> Option<i32> {
            match x {
                None => None,
                Some(i) => Some(i + 1)
            }
        }

        let five = Some(5);
        let six = plus_one(five);
        let none = plus_one(None);

    }

    // catch all (all the rest of the match possibilities)
    {
        let dice_roll = 9;
        match dice_roll {
            3 => println!("Add fancy hat"),
            7 => println!("Remove fancy hat"),
            other => println!("Move player {} spaces.", other) // OTHER!! Similar to ...rest in javascript

        }
    }

    // The common `_` placeholder for other
    {
        let dice_roll = 9;
        match dice_roll {
            3 => println!("Add fancy hat"),
            7 => println!("Remove fancy hat"),
            // _ => () // when nothing happens. Empty tuple () We don't want to run any code in this case
            _ => println!("Re-roll.") // when the `other` is not being used

        }
    }

    // IF LET: Concise control flow
    {
        let config_max = Some(3u8);
        match config_max {
            Some(max) => println!("The maximum configured to be {}", max),
            _ => (), // doing nothing is value is of type Option with a None variant
        }
    }

    // Making the scope above more concise
    {
        let config_max = Some(3u8);

        // works just like match but the code in the if let doesn't run if the condition doesn't match the pattern
        if let Some(max) = config_max {
            println!("The maximum is configured to be {}", max)
        }

        // if let is syntax sugar for a match
        // the trade-off is that exhastive checking will be lost (checking all other arms)
    }

    // only calling out states from Quarters and counting non-quarters
    {
        #[derive(Debug)]
        enum UsState {
            Alabama,
            Alaska, 
        }

        enum Coin {
            Penny,
            Nickle,
            Dime,
            Quarter(UsState),
        }

        let coin = Coin::Quarter(UsState::Alabama);
        let mut count = 0;
        // match coin {
        //     Coin::Quarter(state)  => println!("State quarter from {:?}", state),
        //     _ => count += 1
        // }

        if let Coin::Quarter(state) = coin {
            println!("State quarter from {:?}", state)
        } else {
            count += 1;
        }
    }
}
