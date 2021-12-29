use std::env;
use std::fs;


struct Config {
    filename: String,
    query: String
}

impl Config {

    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let filename = args[2].clone();

        return Config {
            filename: filename,
            query: query
        };
    }
}

fn main() {

    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);


    println!("query {}", config.query);
    println!("filenamee {}", config.filename);

    let content = fs::read_to_string(config.filename)
                    .expect("Something went wrong when tried to read file");

    println!("content is {}", content);
}
