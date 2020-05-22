// use std::io::stdin;
use rand::Rng;

fn main() {
    // println!("Guess the number!");

    // println!("Please input your guess.");

    // let mut guess = String::new();

    // stdin()
    //     .read_line(&mut guess)
    //     .expect("Failed to read line");

    let num = rand::thread_rng().gen_range(0, 301);

    println!("You guessed: {}", num);
}
