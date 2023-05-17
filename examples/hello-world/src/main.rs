use ansi_term::Colour::Red;

fn add(a: isize, b: isize) -> isize {
    a + b
}

fn main() {
    let result = add(3, 7);
    println!("{}", Red.paint(result.to_string()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds() {
        assert_eq!(add(3, 7), 10);
        assert_eq!(add(-3, 7), 4);
        assert_eq!(add(-3, -7), -10);
        assert_eq!(add(3, -7), -4);
    }
}

// mutability
fn variables_example() {
    let x = 10;
    // x = x + 1;
}

// ownership heap allocated types
fn strings_example() {
    let s1 = String::from("hello");
    // let s2 = s1; // shallow copy and invalidate s1, avoid dangling pointers + double frees
    let s2 = s1.clone(); // deep copy

    println!("s1: {:?}", s1);
}

// ownership of stack allocated types
fn integers_example() {
    // no difference between deep and shallow copy here
    let x = 5;
    let y = x;

    println!("x: {:?}", y);
}

fn take_ownership(s: String) {
    println!("s: {:?}", s);
}

// ownership in functino parameters
fn function_ownership_example() {
    let s = String::from("hello");
    take_ownership(s);
    // println!("s: {:?}", s);
}

// references: like pointers, but always valid.
// refers to a string, but does not own it -> we say: print_string borrows a String
fn print_string(s: &String) {
    println!("{}", s);
}

fn reference_example() {
    let s = String::from("hello");
    print_string(&s);
}

fn add_world_suffix(s: &mut String)  {
    s.push_str("world");
}

fn mut_ref_example() {
    let mut s = String::from("hello ");
    add_world_suffix(&mut s);
}

// no two mutable references
// but n non mutable references
fn two_mutable_refs() {
    // prevents data race in compile time
    let mut s = String::from("hello");
    let a = &mut s;
    let b = &mut s;

    let a = &s;
    let b = &s;

    let a = &s;
    let b = &mut s;
}

// scope of a reference: ends when it is last used
fn reference_scope() {
    let mut s = String::from("hello");

    let r1 = &s;
    println!("r1: {:?}", r1);

    let r2 = &mut s;
    println!("r2: {:?}", r2);
}

// fn dangling_pointer() -> &String {
//     let s = String::from("hello");

//     &s
// }

// recap:
// either one mutable reference or any immutable references
// references must be alwas be valid

// type system, a state machine

// find
