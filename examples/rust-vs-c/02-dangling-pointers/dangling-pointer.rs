struct Person {
    name: &String,
    age: u8,
}

impl Person {
    pub fn new(name: &String, age: u8) -> Person {
        let local_name = name.clone();

        Person {
            name: &local_name,
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
    let name = String::from("Jean");
    let p = Person::new(&name, 25);
    println!("{} | {}", p.name(), p.age());

    std::mem::drop(p);
    println!("{} | {}", p.name(), p.age());
}