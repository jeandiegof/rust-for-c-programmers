#[allow(unused_variables)]

fn main() {
    // compiler can infer the type
    let x = 10;

    // or we can specify it
    let y: u8 = 255;

    // we also have floats
    let y: f32 = 124.98;
    let y: f64 = 124.98;

    // string literals
    let s = "hi";

    // characters
    let c = 'z';

    // booleans
    let b = true;

    // n-tuples
    let point_2d = (10, 0);
    let point_3d = (10, 0, 4);

    // arrays
    let some_numbers = [1, 2, 3, 4, 5];
}