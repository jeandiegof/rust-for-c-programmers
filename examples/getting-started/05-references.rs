// Concepts:
//    - passing parameters by references
//    - mutable references
//    - single mut or any immut
fn main() {
    let s1 = String::from("hello");
    print_string(&s1);

    append_word(&s1, " world");
}

fn print_string(s: &String) {
    println!("{}", s);
}

fn append_word(s: &String, word: &str) {
    s.push_str(word);
}

fn single_vs_multiple_references() {
    let mut s1 = String::from("hello");
    let s2 = &s1;

    s1.push_str(" world");
    println!("s2: {}", s2);
}
