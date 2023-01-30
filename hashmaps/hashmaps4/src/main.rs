use std::collections::HashMap;
fn main() {
    let mut hash_map_command = HashMap::new();
    hash_map_command.insert(String::from("Blue"), 50);
    hash_map_command.insert(String::from("White"), 10);
    hash_map_command.insert(String::from("Blue"), 5);

    hash_map_command.entry(String::from("Yellow")).or_insert(1);
    hash_map_command.entry(String::from("White")).or_insert(500);
    for (key, value) in hash_map_command.iter(){
        println!("key - {}, value - {}", key, value);
    }
    let commnd_name = String::from("Blue");
    let result = hash_map_command.get(&commnd_name).copied().unwrap();
    println!("{}", result);



    // let mut hash_map_command = HashMap::new();
    // let string = "b0lt1k dayn dayn dayn dayn b0lt1k majehun neyebok";
    // for word in string.split_whitespace(){
    //     let count = hash_map_command.entry(word).or_insert(0);
    //     *count += 1;
    // }
    // println!("{:?}", hash_map_command);
}
