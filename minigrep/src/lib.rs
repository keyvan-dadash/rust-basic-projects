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

    println!("content is {}", content);


    return Ok(());
}