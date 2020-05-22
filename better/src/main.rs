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

    let y = {
        let x = 2;
        x + 2
    };
    println!(
        "You guessed: {}--{}::{}, {} {} {}",
        num, PI, a, tup.0, array[2], arr[2]
    );


    fun_now();
    tell_age(30);
    println!("{}", y);

    println!("{}", add_two(3, 30));
}

fn fun_now() {
    println!("wow!!!")
}

fn tell_age(age: u8) {
    println!("{}", age);
}

fn add_two(x : i8, y : i8) -> i8 {
    x + y
}