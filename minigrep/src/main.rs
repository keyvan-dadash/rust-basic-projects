use std::env;
use std::fs;
use std::process;
use std::error::Error;

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

fn run(config: Config) -> Result<(), Box<dyn Error>> {

    println!("query {}", config.query);
    println!("filenamee {}", config.filename);

    let content = fs::read_to_string(config.filename)?;

    println!("content is {}", content);


    return Ok(());
}

fn main() {

    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("we have some problem with configs: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }

}
