use std::fs;
use std::error::Error;

pub struct Config {
    filename: String,
    query: String
}

impl Config {

    pub fn new(args: &[String]) -> Result<Config, &str> {

        if args.len() < 3 {
            return Err("we need at least 3 argument!");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        return Ok(Config {
            filename: filename,
            query: query
        });
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {

    println!("query {}", config.query);
    println!("filenamee {}", config.filename);

    let content = fs::read_to_string(config.filename)?;

    for line in search(&config.query, &content) {
        println!("{}", line);
    }

    // println!("content is {}", content);


    return Ok(());
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    
    let mut result = Vec::new();
    
    for line in content.lines() {

        if line.contains(query) {
            result.push(line);
        }
    }
    
    return result;
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
}