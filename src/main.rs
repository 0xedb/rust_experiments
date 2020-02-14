use std::io;

fn main() {
    println!("Enter Guessed Number: ");
    let mut guess = String::new();
    
    io::stdin().read_line(&mut guess).expect("Failed");
    
    println!("You guessed {}", guess);
}
