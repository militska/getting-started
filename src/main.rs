use std::io;
use rand::Rng;


fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..3);

    println!("{}", secret_number.to_string());
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    if guess.trim().parse::<i32>() == Ok(secret_number) {
        println!("Success");

    }

    println!("You guessed: {}", guess);

}