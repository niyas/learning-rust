
trait Shape {
    fn area(&self) -> u32;
}

#[derive(Debug)]
struct Rectangle {
    x: u32,
    y: u32
}

#[derive(Debug)]
struct Circle {
    radius: f64
}

impl Shape for Rectangle {
    fn area(&self) -> u32 {
        self.x * self.y
    }
}

impl Shape for Circle {
    fn area(&self) -> u32 {
        (3.14 * (self.radius * self.radius)) as u32
    }
}

fn main() {
    let c = Circle {radius: 12.6};
    let r = Rectangle {x: 20, y:34};

    println!("Circle: {}, Rectangle: {}", c.area(), r.area());
}