
fn division(x: f64, y: f64) -> Option<f64> {
    if y == 0.0 {
        None
    } else {
        Some(x / y)
    }
}

fn main() {
    let div = division(5.0, 0.0);
    match div {
        Some(x) => println!("{:.4}", x),
        None => println!("Cannot divide by 0")
    }
}