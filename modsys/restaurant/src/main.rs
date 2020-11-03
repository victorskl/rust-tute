// this source file is the default binary crate by convention using file name `main.rs`

// when bringing in structs, enums, and other items with use, it's idiomatic to specify the full path
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
    let val = map.get(&1);
    println!("restaurant main binary called => {}", val.unwrap());
}
