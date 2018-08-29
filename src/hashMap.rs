use std::collections::HashMap;

fn main() {
    let mut hm = HashMap::new();

    hm.insert(String::from("Niyas"), 30);
    hm.insert(String::from("Sumi"), 28);
    hm.insert(String::from("Azhaan"), 3);

    for (k, v) in &hm {
        println!("{} -> {}", k, v);
    }

    match hm.get(&String::from("Azhaan")) {
        Some(&age) => println!("Azhaan's Age is {}", age),
        _ => println!("No key found")
    }
}