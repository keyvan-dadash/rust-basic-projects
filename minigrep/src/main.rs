use std::env;
use std::fs;
use std::process;

struct Config {
    filename: String,
    query: String
}

impl Config {

    fn new(args: &[String]) -> Result<Config, &str> {

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

fn main() {

    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("we have some problem with configs: {}", err);
        process::exit(1);
    });


    println!("query {}", config.query);
    println!("filenamee {}", config.filename);

    let content = fs::read_to_string(config.filename)
                    .expect("Something went wrong when tried to read file");

    println!("content is {}", content);
}
