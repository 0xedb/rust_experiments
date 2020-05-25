
fn main() {
    let mut name = String::from("bruno");
    let c : i8 = 33;
    own(&mut name, c);

    struct Human {
        name : String,
        age : i128
    }

    let bruno = Human {
        name : String::from("edoh"),
        age: 10
    };

    println!("{} {}", bruno.name, bruno.age);
}


fn own(a : &mut String, b : i8) {
    // nothhing serious
    *a = String::from("kofi");
    let v = b;
    println!("{}::{}", a, v)
}