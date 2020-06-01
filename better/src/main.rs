fn main() {
    let status  = true;
    let age : i128 = 0o2323111;
    let tup : (i8, bool) = (9, false);
    let arr = [9; 5];

    println!("hello world {} {}", age, status);
    println!("{} {}", tup.0, tup.1);
    println!("{}", arr[3]);
}