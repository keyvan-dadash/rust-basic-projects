use std::env;
use std::fs;

fn main() {

    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    println!("query {}", query);
    println!("filename {}", filename);

    let content = fs::read_to_string(filename)
                    .expect("Something went wrong when tried to read file");

    println!("content is {}", content);
}
