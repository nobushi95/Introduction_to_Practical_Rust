use std::collections::HashMap;

fn add_world_or_default(key: char, map: &mut HashMap<char, String>) {
    // mapの借用
    match map.get_mut(&key) {
        Some(value) => value.push_str(", World !!"), // get_mutの戻り値をvalueで束縛するので、mapを借用
        None => {
            // get_mutですでにmapは借用されているので、insertできないように見えるが
            // get_mutの戻りがNoneで、何も束縛していないので、insertはmapを借用できる
            map.insert(key, Default::default());
        }
    }
}

fn main() {
    let mut map = HashMap::new();
    map.insert('h', "Hello".to_string());
    add_world_or_default('h', &mut map);
    add_world_or_default('g', &mut map);
    for e in map {
        println!("e: {:?}", e);
    }
}
