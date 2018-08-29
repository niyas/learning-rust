use std::mem;


fn main() {
    let t = (1, 'a', false);
    println!("{:#?}", t);

    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    let sl = &xs[2..5];

    println!("{:?} {:?} {}", xs, sl, mem::size_of_val(&sl));

    let str1 = "Hello ".to_string();
    let str2 = "World".to_string();
    let concat = str1 + &str2;

    println!("{}", concat); 
}