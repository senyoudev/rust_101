fn main() {
    //ownership part
        // variable scope
    let s = String::from("hello");
    // s is valid from this point forward
    // do stuff with s
    println!("{}", s);

} // this scope is now over, and s is no longer valid
