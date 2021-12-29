use std::fs;
use std::error::Error;

pub struct Config {
    pub filename: String,
    pub query: String,
    pub is_case_sensitive: bool,
}

impl Config {

    pub fn new(args: &[String]) -> Result<Config, &str> {

        if args.len() < 3 {
            return Err("we need at least 3 argument!");
        }

        let mut is_case_sensitive = true;

        if args.len() == 4 {
            let case_sensitive = args[3].clone();

            if case_sensitive.eq("case") {
                is_case_sensitive = false;
            }
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        return Ok(Config {
            filename: filename,
            query: query,
            is_case_sensitive: is_case_sensitive,
        });
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {

    println!("query {}", config.query);
    println!("filenamee {}", config.filename);

    let content = fs::read_to_string(config.filename)?;

    type FuncType<'a> = fn(
        query: &str, 
        content: &'a str,
    ) -> Vec<&'a str>;

    let func: FuncType;

    if config.is_case_sensitive {
        func = search;
    } else {
        func = search_case_insensitive;
    }

    for line in func(&config.query, &content) {
        println!("{}", line);
    }

    // println!("content is {}", content);


    return Ok(());
}

pub fn search<'a>(
    query: &str, 
    content: &'a str,
) -> Vec<&'a str> {
    
    let mut result = Vec::new();
    
    for line in content.lines() {

        if line.contains(query) {
            result.push(line);
        }
    }
    
    return result;
}

pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {
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
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}