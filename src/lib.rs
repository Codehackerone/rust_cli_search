use std::fs;
use std::error::Error;

// Function to perform a search on a given file (returns Result type: Either a success with no value,
//                                 or an Err variant containing a dyn Error object with detail about the error)
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {

    // Read the file contents into a string variable, or return early with an error message with detail about the error
    let contents = fs::read_to_string(&config.filename)?;
    // Print the file content
    for line in search(&config.query, &contents) {
        println!("{}", line);
    }

    // Return nothing for success.
    Ok(())
}

// Configuration struct that stores options taken from command line arguments
pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    // Constructor function that returns a Config object or an error message if insufficient arguments are provided
    pub fn new(args: &[String]) -> Result<Config, String> {
        if args.len() < 3 {
            return Err("Not enough arguments!".into());
        }

        let query = String::from(&args[1]);
        let filename = String::from(&args[2]);

        Ok(Config { query, filename })
    }
}

// Method that searches the input contents for the specified query and returns any matching lines
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

// Method that searches the input contents for the specified query in case-insensitive manner and returns any matching lines
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "fast";
        let contents = "\
Rust:
safe, fast, productive.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }

    #[test]
    fn case_sensitive() {
        let query = "ruSt";
        let contents = "\
Rust:
safe, fast, productive.
";

        // Test that the search method is case-sensitive 
        assert_eq!(
          vec!["Rust:"],
          search(query, contents)
        );

        // Test that the search_case_insensitive method searches in a case-insensitive manner
        assert_eq!(
          vec!["Rust:", "no"],
          search_case_insensitive(query, contents)
        );     
  }
}
