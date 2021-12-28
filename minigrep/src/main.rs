use std::env;


fn main() {

    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];


    println!("query {}", query);
    println!("filename {}", filename);
}
