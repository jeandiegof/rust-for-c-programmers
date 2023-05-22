#[derive(Debug)]
enum MathError {
    DivisionByZero,    
}

fn div(a: u32, b: u32) -> Result<f32, MathError> {
    if b == 0 {
        Err(MathError::DivisionByZero)
    } else {
        Ok(a as f32 / b as f32)
    }
}

fn main() {
    let division = div(5, 10);

    match division {
        Ok(value) => println!("{}", value),
        Err(error) => println!("{:?}", error),
    }

    let division = div(5, 0);

    match division {
        Ok(value) => println!("{}", value),
        Err(error) => println!("{:?}", error),
    }
}