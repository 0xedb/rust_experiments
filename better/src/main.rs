fn main() {
    let mut v = 13;

    struct Human {
        name : String,
        age : i16,
    }


    println!("{}", v);
    borrower(&mut v);
    println!("{}", v);
 
    let bruno = Human {
        name : String::from("bruno edoh"),
        age : 39
    };

    println!("{}", bruno.name);
}

fn borrower(i : &mut i8) {
    *i = *i + 11;
}  