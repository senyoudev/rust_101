use std::env;
use std::process;
use cli_project::Config;



fn main() {


    let config = Config::new(env::args()).unwrap_or_else(|err | {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    
    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);
    if let Err(e) = cli_project::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
   
}



