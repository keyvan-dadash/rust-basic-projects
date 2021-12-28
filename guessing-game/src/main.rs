use std::io;
use std::cmp::Ordering;
use rand::Rng;


fn main() {
    println!("Guessing Game");

    let secret_number = rand::thread_rng().gen_range(1..100);

    println!("Please enter your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Cannot read line");


    let guess: u32 = guess.trim().parse().expect("Type a number!");

    print!("You Guessed: {}", guess);

    // for {
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => println!("You win"),
        }
    // }


}
