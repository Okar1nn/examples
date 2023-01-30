fn main() {
    // let data = "Rust is great!".to_string();
    let data = "Rust is great!".to_owned();

    get_char(&data);

    string_uppercase(data);
}

// fn get_char(data: &String) -> char {
fn get_char(data: &str) -> char {
    data.chars().last().unwrap()
}

// fn string_uppercase(mut data: String) {
//     data = data.to_uppercase();

//     println!("{data}");
// }
fn string_uppercase(data: String) {
    println!("{}", data.to_uppercase());
}

