use std::io;


fn main() {
    println!("Guessing Game");

    println!("Please enter your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Cannot read line");

    print!("You Guessed: {}", guess);
}
