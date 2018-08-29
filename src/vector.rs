fn main() {
    let mut v: Vec<i32> = Vec::new(); // For type annotation
    //let mut v = Vec::new(); Without type annotation
    v.push(3);
    v.push(4);
    v.push(5);
    v.push(6);

    for i in &v {
        println!("{}", i);
    }

    v.push(10);

    println!("{:?}, Length: {} Capacity: {}", &v,  v.len(), v.capacity());
    println!("{:?}", v.pop());
}