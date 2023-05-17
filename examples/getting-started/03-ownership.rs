// Concepts:
//    - shallow copy vs deep copy
//    - data has a single owner, always
//    - deep copies are always explicit
//    - ownership in functions

fn main() {
    let s1 = String::from("Hello");
    let s2 = s1
    println!("s1: {}", s1);

    let x = 10;
    let y = x;
    println!("x: {}", x);

    take_ownership(s2);
    println!("s2: {}", s2);
}

fn take_ownership(s: String) {
    println!("Taking ownership of {:?}", s);
}