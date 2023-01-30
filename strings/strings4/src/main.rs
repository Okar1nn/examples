fn string_slice(arg: &str) {
    println!("{arg}");
}
fn string(arg: String) {
    println!("{arg}");
}

fn main() {
    string_slice("blue");
    // string("red".to_string());
    string("red".to_owned());
    // string(String::from("hi"));
    string("hi".to_owned());
    string("rust is fun!".to_owned());
    // string_slice("nice weather".into());
    string_slice("nice weather");
    // string(format!("Interpolation {}", "Station"));
    string("Interpolation Station".to_owned());
    // string_slice(&String::from("abc")[0..1]);
    string_slice(&"abc"[..1]);
    string_slice("  hello there ".trim());
    // string("Happy Monday!".to_string().replace("Mon", "Tues"));
    string("Happy Monday!".replace("Mon", "Tues"));
    string("mY sHiFt KeY iS sTiCkY".to_lowercase());
}

