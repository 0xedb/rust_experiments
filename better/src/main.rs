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
    say(10);

    let mut name = String::from("bruno");
    name.push_str("@theBashShell");

    println!("{}", name);
    let s1 = String::from("hello");
    let s2 = s1;

    println!("{}, world!", s2);
    let s = String::from("bruno");
    let i = 80;

    own(s);
    own1(i);
    // println!("{}", s);
    println!("{}", i);
}



fn own(s : String) {
    println!("I OWN {}", s);
}

fn own1(i : i8) {
    println!("{}", i);
}



/////////////////////////









fn fun_now() {
    println!("wow!!!")
}

fn tell_age(age: u8) {
    println!("{}", age);
}

fn add_two(x : i8, y : i8) -> i8 {
    x + y
}


fn say(i : i16) {
    if i > 10 {
        println!("greater than 10");
    } 
    else if i == 10 {
        println!("TEN!!");
    }
    else {
        println!("greater than 10");
    }
}