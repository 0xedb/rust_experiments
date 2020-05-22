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

    const PI: f64 = 3.14;

    let a = "go".to_uppercase();

    let tup: (char, i8) = ('c', 10);
    let array: [i32; 3] = [1, 2, 3];
    let arr = [0; 3];

    println!(
        "You guessed: {}--{}::{}, {} {} {}",
        num, PI, a, tup.0, array[2], arr[2]
    );
}
