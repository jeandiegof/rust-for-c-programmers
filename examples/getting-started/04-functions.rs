fn add(a: u32, b: u32) -> u64 {
    a as u64 + b as u64
}

fn say_hello() {
    println!("Hello");
}

fn main() {
    let x = add(u32::MAX, u32::MAX);
    println!("{:x}", x);
}