use std::env;
use std::fs;

fn main() {
    let args : Vec<String> = env::args().collect(); // this line for command line arguments

    if args.len() != 3 {
        println!("Wrong number of arguments");
        std::process::exit(1);
    }

    let query = &args[1];
    let filename = &args[2];
    
    println!("Searching for {}", query);
    println!("In file {}", filename);
    
    let contents = fs::read_to_string(filename)
        .expect("Something Went Wrong");

    println!("With text:\n{}", contents);
}
