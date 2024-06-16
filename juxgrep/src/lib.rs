use std::fs;
use std::error::Error;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents: String = fs::read_to_string(config.filename)?;
    println!("With text:\n{}", contents);
    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments")
        }
    
        // In Rust the ownership of the object will change if you just assign it to another variable. Thus, we need to clone here, and also why we prefer to work with pointers
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { query, filename })
    }
}

pub fn search(query: &str, contents: &str) -> Vec<&str> {
    vec![]
}

// This is how tests are written for a module in Rust
#[cfg(test)]
mod tests {
    use super::*;

    // This is an attribute, similar to decorators?
    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
