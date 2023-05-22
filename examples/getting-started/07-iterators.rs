fn main() {
    let numbers = [1, 54, 9, 13, 43, 12, 18];

    numbers
        .iter()
        .for_each(|e| println!("{:?}", e));

    let even = numbers
                    .iter()
                    .cloned()
                    .filter(|e| *e % 2 == 0)
                    .collect::<Vec<u8>>();
    println!("even numbers: {:?}", even);

    let squares = numbers
                        .iter()
                        .cloned()
                        .map(|e| e as u16 * e as u16)
                        .collect::<Vec<u16>>();
    println!("squares: {:?}", squares);
}