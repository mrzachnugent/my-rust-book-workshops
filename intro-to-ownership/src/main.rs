fn main() {
    // MEMORY ALLOCATION
    // Memory types: Stack Vs Heap
    // Stack => data known size, fast, LIFO (automatically pushess and pops data)
    // Heap => unknown size, slower, stores pointer on Stack (uses Ownership to keep track of and cleans up memory that isn’t being used anymore)
    chapter4_1();

    // Rust has a feature for using a value without transferring ownership, called references
    chapter4_2();

    // The Slice Reference Type
    chapter4_3()
}

fn chapter4_1() {
    {
        // println!("{}", s); _s is not found in this scope
        let _s = "hello"; // _s is valid starting here
    }
    // _s is no longer in scope

    let howdy = "Howdy"; // String literrals are immutible
                         //You can create a mutatable String from a string literal:
    let mut s = String::from(howdy); // String::from() requests unknown amount of memory from Heap at compile time but  requests the memory it needs at runtime
    s.push_str(", Universe!");
    println!("{}", s);

    // it’s OUR RESPONSIBILITY to identify when memory is no longer being used and call code to explicitly return it
    // If we forget, we’ll waste memory.
    // If we do it too early, we’ll have an invalid variable.
    // If we do it twice, that’s a bug too.
    // We need to pair exactly one allocate with EXACTLY one free.

    // RUST AND OWNERSHIP
    // Memory is automatically returned once the variable that owns it goes out of scope.

    {
        let _h = String::from("hello"); // h is valid from this point forward
                                        // do stuff with h
    }
    // this scope is now over, and h is no longer valid

    // How does being out of scope affect memory?
    // When a variable goes out of scope,
    // Rust calls a special function for us.
    // This function is called DROP (drop), and it’s where the author of String can put the code to return the memory.
    // Rust calls drop AUTOMATICALLY at the closing curly bracket.

    // Ways Variables and Data Interact: Move
    {
        // Example 1
        let x = 5; // stores data on the stack
        let _y = x; // makes a copy of the value of x which store 5 again on the stack

        // Example 2
        let s1 = String::from("hello"); // stores (pointer to the Heap, length, capacity)
        let s2 = s1; // also stores (pointer to the same memory address as s1, same length as s1, and same capacity as s1)

        // If Rust copied values like in example 1,
        // the operation s2 = s1 could be very expensive in terms of runtime performance
        // if the data on the heap were large.

        // when s2 and s1 go out of scope, they will both try to free the same memory.
        // This is known as a double free error and is one of the memory safety bugs we mentioned previously.
        // Freeing memory twice can lead to memory corruption, which can potentially lead to security vulnerabilities.

        //println!("{}", s1); //value borrowed by s2 so it is no longer the owner of the (pointer to the Heap, length, capacity)
        println!("{}", s2);
    }

    // Example 2.1
    {
        let a1 = String::from("I'm going to MOVE from a1 to _a2");
        let _a2 = a1; // Since Rust invalidates a1, instead of a shallow copy, it’s known as a MOVE
                      // a1 MOVED into _a2
                      // Only _a2 is valid so when it goes out of scope, it will free the memory (not a1), and we’re done.

        // Rust will never automatically create “deep” copies of your data.
        // Therefore, any automatic copying can be assumed to be inexpensive in terms of runtime performance.
    }

    // Deep copy on the Heap of a String
    {
        let s1 = String::from("Hey again");
        let s2 = s1.clone();

        println!("{} and {} both valid still", s1, s2)
    }

    // Deep copy on the Stack of an interfer
    {
        let x = 5;
        let y = x;

        println!("x = {}, y = {}", x, y);
    }

    // Integers that have a known size at compile time are stored entirely on the stack,
    // so copies of the actual values are QUICK to make.
    // No need for a 'clone' method

    // Functions
    // Passing a variable to a function will move or copy, just as assignments

    {
        let s = String::from("Sup");

        take_ownership(s);

        // println!("{}", s); NO WORK: s was moved to the function take_ownership

        let i = 2;

        make_a_copy(i);

        println!("Original value: {}", i); //works because it made a copy, so make_a_copy did not take ownership of i
    }

    // Return Values and Scope
    {
        let s1 = give_ownership();
        let s2 = String::from("hello");
        let s3 = takes_and_gives_back(s2);

        println!(
            "s1 recieved ownership of '{}' from the give_ownership function",
            s1
        );
        println!(
            "s2 gave ownership to takes_and_gives_back and moved it so s3: {}",
            s3
        );
    }

    // Rust let us return multiple values using a tuple
    {
        let s1 = String::from("hello");

        let (s2, len) = calculate_length(s1);

        println!("The length of '{}' is {}.", s2, len);
    }
}

fn take_ownership(some_string: String) {
    println!("{}", some_string);
}

fn make_a_copy(some_int: i32) {
    println!("Copied value: {}", some_int)
}

fn give_ownership() -> String {
    let hand_me_down = String::from("yours");
    hand_me_down
}

fn takes_and_gives_back(some_string: String) -> String {
    some_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}

fn chapter4_2() {
    // References and Borrowing
    // A reference is like a pointer in that it’s an address we can follow to access data stored at
    // that address that is owned by some other variable.
    // Unlike a pointer, a reference is GUARANTEED to point to a valid value of a particular type.
    {
        let s1 = String::from("hello darkness my old friend");

        let len = calculate_length_with_ref(&s1); // using &s1 does not transfer ownership, only provided a reference to value of s1

        println!(
            "The length of '{}' is {} characters in the String.",
            s1, len
        );
    }

    // Modifying with refs
    {
        // mutable variable
        let mut hello = String::from("HELLO");
        // pass a mutable reference to fn
        change_val_from_ref(&mut hello);
        println!("{}", hello);
    }

    // You can have only ONE MUTABLE REFERENCE to a PARTICULAR PIECE OF DATA at a time.
    // The benefit of having this restriction is that Rust can prevent data races at compile time
    {
        let mut s = String::from("Something original");
        {
            let r1 = &mut s;
            // let r2 = &mut s; // NO WORK
            // let r3 = &s; // NO WORK cannot have mutable and immutable refs in same scope neither
            println!("{}", r1);
        }
        // New scopes allow for multiple mutable references, just not simultaneous ones:
        let r2 = &mut s; // Works :D
        println!("{}", r2);

        // multiple immutable references are allowed.
        // No one who is just reading the data has the ability to affect anyone else’s reading of the data
    }

    // Making the last of multiple refs mutable WORKS :D
    {
        let mut s = String::from("MULTIPLE REFS");

        let r1 = &s; // no problem
        let r2 = &s; // no problem
        println!("{} and {}", r1, r2);
        // variables r1 and r2 will not be used after this point

        let r3 = &mut s; // no problem
        println!("{}", r3);
    }

    // In Rust, the compiler guarantees that references will never be dangling references:
    {
        // fn dangle() -> &String { // A function cannot return a ref without providing an argument
        //     let s = String::from("dangle me");

        //     &s
        // } // NO WORK

        let s = no_dangle();
        println!("{}", s)
    }
}

// Example of BORROWING:
// References are immutable, We’re not allowed to modify something we have a reference to
fn calculate_length_with_ref(s: &String) -> usize {
    // argument NEEDS to be typed as a reference
    s.len()
}
// Since s is a reference, the drop function is not called (since it is not the owner), so nothing happens

fn change_val_from_ref(s: &mut String) {
    s.push_str(", world!");
}

fn no_dangle() -> String {
    let s = String::from("no dangle");

    s // returns String instead of a &String
}

fn chapter4_3() {
    // Slice is a reference type so it does not have ownership
    // Error prone without using slice
    {
        let mut s = String::from("hello world");

        let _word = first_word_index_without_slice(&s); // word will get the value 5

        s.clear(); // this empties the String, making it equal to ""

        // _word still has the value 5 here, but there's no more string that
        // we could meaningfully use the value 5 with. word is now totally invalid!
    }

    // A string slice is a reference to part of a String, and it looks like this:
    {
        let s = String::from("hello world");
        let len = s.len();
        let _hello = &s[0..5];
        let hello = &s[..5]; // same as above ^^
        let _world = &s[6..11];
        let world = &s[6..len]; // works as above ^^
        let _whole = &s[0..len];
        let _whole = &s[..]; // workds as above ^^

        println!("USING STRING SLICE: {}, {}!", hello, world);
    }

    // Prevent mutability error with Slice
    // If we have an immutable reference to something, we cannot also take a mutable reference
    // .clear() needs to get a mutable reference
    {
        let mut _s = String::from("hello world");

        let word = first_word(&_s);

        //s.clear(); // error!
        // Rust disallows the mutable reference in clear and
        // the immutable reference in word from existing at the same time,

        println!("the first word is: {}", word);
    }

    // String literals are String Slices
    {
        let my_string = String::from("hello world");
        // `first_word_improved` works on slices of `String`s, whether partial or whole
        let _word = first_word_improved(&my_string[0..6]);
        let _word = first_word_improved(&my_string[..]);
        // `first_word_improved` also works on references to `String`s, which are equivalent
        // to whole slices of `String`s
        let _word = first_word_improved(&my_string);

        let my_string_literal = "Hello World!! (I'm immutable because I'm a String Slice";

        // `first_word_improved` works on slices of string literals, whether partial or whole
        let _word = first_word_improved(&my_string_literal[0..6]);
        let _word = first_word_improved(&my_string_literal[..]);

        // Because string literals *are* string slices already,
        // this works too, without the slice syntax!
        let word = first_word_improved(my_string_literal);

        println!("{}", word);
    }

    // Other SLICES
    // Array Slice
    {
        let a = [1, 2, 3, 4, 5];

        let slice = &a[1..3]; // has type of &[i32]

        assert_eq!(slice, &[2, 3]);
    }

    // More Slices to come!
}

fn first_word_index_without_slice(s: &String) -> usize {
    let bytes = s.as_bytes(); // converts string to bytes to check if there is a space character

    // deconstruct tuple in for loop
    for (i, &item) in bytes.iter().enumerate() {
        // loops over bytes by making them iterable, then converting it to a tuple (index, element reference)
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word(s: &String) -> &str {
    // “string slice” is written as &str
    // Similar to funciton above ^^
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn first_word_improved(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
