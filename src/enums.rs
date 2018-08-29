#![allow(dead_code)]

#[derive(Debug)]
enum Direction {
    Up(Point),
    Down(Point),
    Left(Point),
    Right(Point)
}

#[derive(Debug)]
enum Keys {
    UpKey(String),
    DownKey(String),
    LeftKey(String),
    RightKey(String)
}

enum Shape {
    Rectangle {width: u32, height: u32},
    Square(u32),
    Circle(f64)
}


impl Shape {
    fn area(&self) -> f64 {
        match *self {
            Shape::Rectangle {width, height} => (width * height) as f64,
            Shape::Square(ref s) => (s * s) as f64,
            Shape::Circle(ref r) => 6.14 * (r * r)
        }
    }
}
impl Direction {
    fn match_direction(&self) -> Keys {
        match *self {
            Direction::Up(_) => Keys::UpKey(String::from("Pressed w")),
            Direction::Down(_) => Keys::DownKey(String::from("Pressed s")),
            Direction::Left(_) => Keys::LeftKey(String::from("Pressed a")),
            Direction::Right(_) => Keys::RightKey(String::from("Pressed d"))
        }
    }
}

impl Keys {
    fn destruct(&self) -> &String {
        match *self {
            Keys::UpKey(ref s) => s,
            Keys::DownKey(ref s) => s,
            Keys::LeftKey(ref s) => s,
            Keys::RightKey(ref s) => s,
        }
    }
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32
}

fn main() {
    let u = Direction::Up(Point {x: 10, y:4});
    let k = u.match_direction();
    println!("{}", k.destruct());
    println!("======================== Calculating Area =============");

    let _r = Shape::Rectangle {width: 10, height: 15};
    let _s = Shape::Square(10);
    let _c = Shape::Circle(12.2);

    let ar = _r.area();
    let aa = _s.area();
    let ac = _c.area();

    println!("Area of Circle: {:.4}", ac);
    println!("Area of Rectangle: {}", ar);
    println!("Area of Square: {}", aa);
}