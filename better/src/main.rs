
fn main() {
    let mut name = String::from("bruno");
    let c : i8 = 33;
    own(&mut name, c);

    struct Human {
        name : String,
        age : i128
    }

    struct Color(i8, i8);

    let bruno = Human {
        name : String::from("edoh"),
        age: 10
    };

    let mut anon = Human {
        name : String::from("anon"),
        age: 101
    };

    let color = Color(10,120);

    anon.name = String::from("agessss");
    println!("{} {}", bruno.name, bruno.age);
    println!("{} {}", anon.age, anon.name);
    println!("{} {}", color.0, color.1);
}


fn own(a : &mut String, b : i8) {
    // nothhing serious
    *a = String::from("kofi");
    let v = b;
    println!("{}::{}", a, v)
}