use std::collections::HashMap;

macro_rules! compile_time_hash_map {
    ($($key:expr => $value:expr), *) => {{
        let mut map = HashMap::new();
        $(map.insert($key, $value);)*
        map
    }};
}

fn main() {
    let my_map = compile_time_hash_map! {
        "key1" => 42,
        "key2" => 123,
        "key3" => 798
    };

    println!("Value for key 'key2': {:?}", my_map.get("key2"));
    println!("Value for key 'key3': {:?}", my_map.get("key3"));
}
