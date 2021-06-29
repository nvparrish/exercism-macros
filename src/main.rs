use macros::hashmap;
use std::collections::HashMap;

fn test_trailing_comma() {
    let mut expected = HashMap::new();
    expected.insert('h', 89);
    expected.insert('a', 1);
    expected.insert('s', 19);
    expected.insert('h', 8);
    /*assert_eq!(
        hashmap!(
            'h' => 89,
            'a' => 1,
            's' => 19,
            'h' => 8,
        ),
        expected
    ); */
}

fn main() {
    println!("Hello, world!");
    //let computed: HashMap<u32, u32> = hashmap!();
    let without_comma = macros::hashmap!(23=> 623, 34 => 21);
    for (key, value) in &without_comma {
        println!("{}: {}", key, value);
    }
    test_trailing_comma();
}