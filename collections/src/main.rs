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

}
