
#[derive(Debug)]
struct Fib {
    c: u32,
    n: u32
}

impl Iterator for Fib {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        let n = self.c + self.n;
        self.c = self.n;
        self.n = n;

        Some(self.c)
    }
}

fn fib() -> Fib {
    Fib {c:1, n:1}
}

fn main() {
    for i in fib().take(10) {
        println!("{}", i);
    }
    println!("=========================");
    for i in fib().skip(5).take(10) {
        println!("{}", i);
    }
}