struct Person {
    name: String,
    age: u8,
}

impl Person {
    pub fn new(name: &String, age: u8) -> Person {
        Person {
            name: name.clone(),
            age
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn age(&self) -> u8 {
        self.age
    }
}


fn main() {
    let p = Person::new(&String::from("Jean"), 25);
    println!("{} | {}", p.name(), p.age());

    std::mem::drop(p);
    println!("{} | {}", p.name(), p.age());
}