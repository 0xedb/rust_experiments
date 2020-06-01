fn main() {
    let status  = true;
    let age : i128 = 0o2323111;
    let tup : (i8, bool) = (9, false);
    let arr = [9; 5];

    println!("hello world {} {}", age, status);
    println!("{} {}", tup.0, tup.1);
    println!("{}", arr[3]);
    println!("{}", add_two(10, 39));

    println!("{}", checker(99));
    println!("{} here", looper(3));
}

fn add_two(first : i16, second : i16) -> i16 {
    first + second
}

fn checker(num : i8) -> i8 {
    let y =     if num > 10 {
        num + 1
    } else {
        num - 1
    };

    y
}

fn looper(mut num: i8) -> i8 {
    loop {
        println!("{}", num);
        num = num + 1;

        if num == 20 {
            break num
        }
    }
}