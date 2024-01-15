use std::env;
use std::fs;

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Config {

        if args.len() != 3 {
            println!("Wrong number of arguments");
            std::process::exit(1);
        }
    
        let query = args[1].clone();
        let filename = args[2].clone();
        Config {query, filename}
    }
}

fn main() {
    let args : Vec<String> = env::args().collect(); // this line for command line arguments


    let config = Config::new(&args);
    
    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);
    
    let contents = fs::read_to_string(config.filename)
        .expect("Something Went Wrong");

    println!("With text:\n{}", contents);
}


