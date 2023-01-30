fn main() {
    let mut res = 42;
    let option = Some(12);
    // for x in option {
    if let Some(x) = option {
        res += x;
    }
    println!("{res}");
}
