use std::env;
use std::process;

use io_project::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err|{
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    
    println!("Searching for the: {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = io_project::run(config) {
        eprintln!("Application error : {}", e);

        process::exit(1);
    }
}





