use std::env;
use std::fs;
use std::process;

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {

        if args.len() < 3 {
            return Err("Invalid Number of Arguments");
        }
    
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config {query, filename})
    }
}

fn main() {
    let args : Vec<String> = env::args().collect(); // this line for command line arguments


    let config = Config::new(&args).unwrap_or_else(|err | {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    
    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);
    
    let contents = fs::read_to_string(config.filename)
        .expect("Something Went Wrong");

    println!("With text:\n{}", contents);
}


