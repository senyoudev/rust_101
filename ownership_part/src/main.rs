fn main() {
    //Ownership rules
        // Each value in Rust has a variable that’s called its owner.
        // There can only be one owner at a time.
        // When the owner goes out of scope, the value will be dropped.
    //ownership part
        // variable scope
    let s = String::from("hello");
    // s is valid from this point forward
    // do stuff with s
    println!("{}", s);

    //memory && allocation : move
    let s1 = String::from("hello");
    let s2 = s1; // move s1 to s2, not shallow copy
    //println!("{}, world!", s1); //error
    println!("{}, world!", s2);

    //ownership && functions
    let s3 = String::from("hello"); // s comes into scope
    takes_ownership(s3); // s's value moves into the function...
    // ... and s3 is no longer valid here

    let x = 5; // x comes into scope
    makes_copy(x); // x would move into the function,
                   // but i32 is Copy, so it’s okay to still
                   // use x afterward

    //reference && borrowing
    let s4 = String::from("hello");
    let len = calculate_length(&s4);
    println!("The length of '{}' is {}.", s4, len);
    //mutable reference

    // The rules of references
        // At any given time, you can have either one mutable reference or any number of immutable references.
        // References must always be valid.

    // The Slice Type
    let s5 = String::from("hello world");
    let hello = &s5[..5]; // same as &s5[0..5]

    let first_word : &str = first_word(&s5);
    println!("first word is {}", first_word);


} // this scope is now over, and s is no longer valid

fn first_word (s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return &s[0..i];
        }
    }
    &s[..]
}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.


fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
    } // Here, some_integer goes out of scope. Nothing special happens. 

fn calculate_length(s: &String) -> usize {
    s.len()
}