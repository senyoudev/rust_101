use std::collections::HashMap;

fn main() {
    // Vectors 
        // Create a new empty vector
        // vectors are implemented using generics
        // vectors can only store values of the same type
        // vectors are stored on the heap
        // vectors can be mutated
        let mut v: Vec<i32> = Vec::new();
        // Create a new vector with initial values
        // vec! is a macro that creates a new vector
        // macro is a special type of function , ! indicates it is a macro
        // macro can take a variable number of arguments
        let mut v2 = vec![1, 2, 3];
        // Add values to vector
        v.push(5);
        v2.push(6);
        println!("v = {:?}", v);
        println!("v2 = {:?}", v2);

        // Accessing values in vector
        // 1 - Using index
        let third: &i32 = &v2[2];
        println!("The third element is {}", third);
        // 2 - Using get method
        match v2.get(2) {
            Some(third) => println!("The third element is {}", third),
            None => println!("There is no third element"),
        }

        // Iterating over values in vector
        // 1 - Using for loop
        for i in &v2 {
            println!("{}", i);
        }
        // 2 - Using for loop with mutable reference
        for i in &mut v2 {
            *i += 50;
        }

        // Using an enum to store multiple types
        // 1 - Using enum
        enum SpreadsheetCell {
            Int(i32),
            Float(f64),
            Text(String),
        }
        let row = vec![
            SpreadsheetCell::Int(3),
            SpreadsheetCell::Float(3.14),
            SpreadsheetCell::Text(String::from("Hello")),
        ];
    
    // Strings
        // strings are stored as a collection of UTF-8 encoded bytes
        // strings are stored on the heap
        
        let s = String::new();
        let data = "initial contents";
        let s = data.to_string(); // convert string literal to String
        let s = String::from("initial contents"); // create a String from a string literal

        // Updating a string
        let s1 = String::from("Hello, ");
        let s2 = String::from("world!");
        let s3 = s1 + &s2; // s1 has been moved here and can no longer be used
        let s4 = format!("{}-{}", s2, s3); // s1 is still valid here


        // Indexing into strings
        let hello = "Здравствуйте"; // Russian for hello
        //slice the first character on hello
        let character = &hello[0..2]; // 2 is the number of bytes for the first character 3
        println!("{}", character);

        // slicing strings
        let hello = "Здравствуйте";
        for c in hello.chars() {
            println!("{}", c);
        }

        // iterating over bytes
        for b in hello.bytes() {
            println!("{}", b);
        }

    // Hash Maps
        // Hash maps store a mapping of keys to values
        // create a new empty hash map
        let mut scores = HashMap::new();
        // insert a key-value pair into the hash map
        scores.insert(String::from("Blue"), 10);
        // get a value from the hash map
        let scoreBlue = scores.get(&String::from("Blue")).copied().unwrap_or(0);
        let scoreRed = scores.get(&String::from("Red")).copied().unwrap_or(0);
        println!("scoreBlue = {}", scoreBlue);
        println!("scoreRed = {}", scoreRed);

        // iterate over hash map
        for (key, value) in &scores {
            println!("{}: {}", key, value);
        }

        // Hash maps and ownership
        // values that implement the Copy trait are copied into the hash map

        // updating a hash map
        // overwriting a value
        scores.insert(String::from("Blue"), 25);
        println!("{:?}", scores);
        // only inserting a value if the key has no value
        scores.entry(String::from("Yellow")).or_insert(50);

        // updating a value based on the old value
        let text = "hello world wonderful world";
        let mut map = HashMap::new();
        for word in text.split_whitespace() {
            let count = map.entry(word).or_insert(0);
            *count += 1;
        }
        println!("{:?}", map);

}
