// Import standard libraries
use std::env;
use std::process;

use grep_cli::Config;

// Main function to be executed
fn main() {
    // Get command line arguments
    let args: Vec<String> = env::args().collect();

    // Parse and store configuration from the command line arguments, or print an error message and exit with code 1 if parsing fails
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // Print search query and file name taken from config
    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    // Call the "run" function with the supplied config, or print an error message and exit with code 1 if "run" fails
    if let Err(e) = grep_cli::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
