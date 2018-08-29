#[derive(Debug)]
enum Shape {
    Rectangle {width:u32, height: u32},
    Square(u32),
    Circle(f64)
}
fn main() {
    let v = vec![
        Shape::Rectangle {width: 10, height: 2},
        Shape::Square(13),
        Shape::Circle(23.4)
    ];

    println!("{:?}", v);
}