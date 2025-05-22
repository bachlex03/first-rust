use std::collections::HashMap;

fn main() {
   let mut map = HashMap::new();

   map.insert(0, "zero");
   map.insert(1, "T");

   match map.get(&0) {
    Some(value) => println!("Found: {}", value),
    None => println!("Not found"),
   }

    match map.get(&1) {
     Some(value) => println!("Found: {}", value),
     None => println!("Not found"),
    }


    for (key, value) in &map {
        println!("{}: {}", key, value);
    }

    map.remove(&0);

    println!("{:?}", map);
}
