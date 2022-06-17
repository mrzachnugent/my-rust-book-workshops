// Most common collections:

// 1. Vector   : stores a variable number of values next to each other
//    ------

// 2. String   : a collection of characters.
//    ------

// 3. Hash map : stores associated value with a particular key
//    --------

fn main() {
    println!("Hello, common collections!");

    // Vec<T>
    // Vectors can only store variable values of the same type
    // Good exmaple: shopping cart
    {
        let _v: Vec<i32> = Vec::new(); // must type since we are not inseting any value into this vector here.
    }

    // creating with initial values
    {
        let _v = vec![1, 2, 3];
    }

    // updating
    {
        let mut v: Vec<i32> = Vec::new(); // need mut

        v.push(3);
        v.push(2);
        v.push(1);
    } // <- v goes out of scope and is freed here

    // referencing stored values
    {
        let v = vec![1, 2, 3, 4, 5];

        // 1. With index
        let third: &i32 = &v[2];
        println!("The third element is {}", third);

        // 2. with get method
        match v.get(2) {
            Some(at_index_two) => println!("The element at index 2 is {}", at_index_two),
            None => println!("There is no element at index 2!"),
        }
    }

    {
        let mut v = vec![1, 2, 3];
        let mut r = &v[1];
        // v.push(4); // NO WORK
        println!("{}", v[1]);
        println!("{}", r);
        println!("{}", v[1]);
    }

    {
        let mut v = vec![100, 32, 57];

        for p in &v {
            println!("{}", p)
        }

        for f in &mut v {
            // add 50 to evey element in `v`
            *f += 50; // `*` deference operator to get the value in i before we can use the `+=` operator
        }
        for p in &v {
            println!("{}", p)
        }
    }

    // Using an Enum to store multiple types
    {
        enum SpreadsheetCell {
            Int(i32),
            Float(f64),
            Text(String),
        }

        let row = vec![
            SpreadsheetCell::Int(3),
            SpreadsheetCell::Text(String::from("blue")),
            SpreadsheetCell::Float(10.12),
        ];
    }

    // STRINGS
    {
        let mut s = String::new();

        let data = "initial contents";

        let s = data.to_string();

        let s = "initial contents".to_string();
        // these two ^ ⌄ do the same exact thing
        let s = String::from("initial contents");
    }

    // Updating a string
    {
        let mut s1 = String::from("foo");
        let s2 = "bar";
        s1.push_str(s2); // push_str did not take ownership of s2
        println!("s2 is {} and s1 is {}", s2, s1);
    }

    {
        let mut s = String::from("lo");
        s.push('l'); // push does take ownership of `l`
    }

    // Concatenating with the `+` sign
    {
        let s1 = String::from("Hello, ");
        let s2 = String::from("Mars!");
        let s3 = s1 + &s2 + "wagwan"; // s3 took ownership of s1
                                      //let s3 = &s1 + &s2; // cannot be used to concatenate two `&str` strings, needs at least one String

        // println!("{}", s1) // NO WORK
    }

    {
        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");

        // let s = s1 + "-" + &s2 + "-" + &s3; // `s` takes onership of `s1`

        let s = format!("{}-{}-{}", s1, s2, s3); // format! does not take ownership

        println!("{}", s1)
    }

    // Indexing into Strings
    {
        let s = String::from("word");

        // let first_letter = s[0] // NO WORK

        // WHY NOT?!

        // Strings are stored as VECTORS of `u8` values
        // 3 ways to look at Strings from Rust's perspective as:

        // 1. Bytes

        // 2. Scalar values

        // 3. Grapheme clusters

        // Rust provides different ways of interpreting the raw string data that computers store so that each program can choose the interpretation it needs,
        // no matter what human language the data is in.

        // BASICALLY, the item of index 0 in the vector would return a byte value and not the appropriate letter.
        // Some languages have different letters which adds to the complexity since the amount of bytes per letter can vary
        // Finally, Indexing operations are supposed to ALWAYS take constant time `O(1)` which could not be the case with
        // lettings that use different amounts of bytes. Some letter would take more time so time of operation would not be constant.
    }

    // SLICING STRINGSS
    {
        let hello = "Здравствуйте";

        let s = &hello[0..4]; // here `s` contains the first 4 bytes of the string `hello`

        println!("{}", s); // outputs: `Зд`

        // let d = &hello[0..1]; // NO WORK => Rust will panic at RUNTIME
        // ERROR: byte index 1 is not a char boundary
    }

    // Iterating over Strings
    {
        // For individual Unicode scalar values, use the chars method
        for c in "नमस्ते".chars() {
            println!("{}", c)
        } // Prints 6 chars
    }

    // Alternatively, the bytes method returns each raw byte
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    } // Prints the following 18 bytes:
      // 224
      // 164
      // 168
      // 224
      // 164
      // 174
      // 224
      // 164
      // 184
      // 224
      // 165
      // 141
      // 224
      // 164
      // 164
      // 224
      // 165
      // 135

    // HASH MAPS

    use std::collections::HashMap; //Since not used as much, doesn't automatically come in the prelude like Vec and String

    {
        let mut scores = HashMap::new();

        // all keys must be of a certain type and all values must be of a certain type
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);
    }

    {
        let teams = vec![String::from("MTL"), String::from("ATL")];
        let initial_scores = vec![10, 50];

        let mut scores_per_team: HashMap<_, _> =
            teams.into_iter().zip(initial_scores.into_iter()).collect();
    }

    // Hash Maps and Ownership
    {
        let field_name = String::from("Favorite color");
        let field_value = String::from("Blue");

        let mut map = HashMap::new();
        map.insert(field_name, field_value); // values moved here
                                             // field_name and field_value are invalid at this point, try using them and

        // println!("{}, {}", field_name, field_value) // NO WORK!

        // To avoid transfering ownership, use references as args to `insert`
        // BUT the valuess must stay valid as long as you want to use the HashMap
    }

    // Accessing HashMap values
    {
        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);

        let team_name = String::from("Blue");
        let score = scores.get(&team_name);

        if let Some(score_from_team_name) = score {
            println!("The score from {} is {}", team_name, score_from_team_name)
        }
    }

    {
        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Blue"), 25); // overridess first insert

        println!("{:?}", scores);
    }

    // Only Inserting a Value If the Key Has No Value
    {
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);

        scores.entry(String::from("Yellow")).or_insert(50);
        scores.entry(String::from("Blue")).or_insert(50); // since "Blue" already has a value, it won't get overriden

        println!("{:?}", scores);
    }

    // Updating a Value Based on the Old Value
    {
        let text = "hello world  wonderful world";

        let mut map = HashMap::new();

        for word in text.split_whitespace() {
            // can iterate over each word

            let count = map.entry(word).or_insert(0);
            *count += 1;
        }

        println!("{:?}", map); // prints: {"world": 2, "wonderful": 1, "hello": 1}
    }

    // The default hashing function called `SipHash` is not the fastest hashing algorithm available,
    // but the trade-off for better security that comes with the drop in performance is worth it.

    // Extras
    {
        let mut nums = vec![1, 2, 3, 4];
        let mut nums2 = vec![1, 2, 3, 4, 5];

        println!("{}, {}", median(&mut nums), median(&mut nums2));

        let s = String::from("Transform me to pig latin");
        println!("To pig latin-ish: {}", to_pig_latin(&s))
    }
}

fn median(nums: &mut Vec<i32>) -> i32 {
    nums.sort();
    let middle_index = nums.len() / 2;
    println!("middle_index usize: {}", middle_index); // since not float, cannot have decimals
    nums[middle_index]
}

fn to_pig_latin(sent: &String) -> String {
    let mut og = String::from("");
    for word in sent.split_whitespace() {
        let mut s = String::from("");
        let w_o_first_letter = word.get(1..);
        match w_o_first_letter {
            Some(w) => s += w,
            None => break,
        }
        let first_letter = word.get(0..1);
        if let Some(l) = first_letter {
            s += &format!("{}{}", l, "ay")
        }
        if og.is_empty() {
            og += &s
        } else {
            og += &format!(" {}", s)
        }
    }
    og
}
