use std::fs;
use std::error::Error;
use std::env;

pub struct Config { 
    // grouping conf values
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config { 
    // implementation for Struct using build constructor  
    // error handling         
    // read config from arguments and Env         
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {              // error handling
            return Err("not enough arguments");
        }
        let query = args[1].clone();     // need new refs
        let file_path = args[2].clone(); // clone not eff but easy
        let mut ignore_case = env::var("IGNORE_CASE").is_ok();
        if args.len() > 3 {
            ignore_case = true;
        }

        Ok(Config { 
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // main run
    // read content from file
    // search depends on config
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // search with case
    let mut results = Vec::new(); // using vector for matching lines
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(
    query: &str, 
    contents: &'a str
) -> Vec<&'a str> {
    // search without case
    let query = query.to_lowercase(); //shadowed
    let mut results = Vec::new();     // using vector for matching lines
    
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) { // to_lowercase create new
            results.push(line);                   // data rather than ref
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duck";
        let contents ="\
Rust:
safe, fast, producktive.
Pick three.";

        assert_eq!(vec!("safe, fast, producktive."), search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents ="\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!("Rust:", "Trust me."), search_case_insensitive(query, contents));
    }
}