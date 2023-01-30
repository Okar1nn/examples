fn main() {
    let str1 = String::from("Hello");
    let give = give();
    let s = take_and_give(str1);
    print_value(s);
    print_value(give);
    let ssilka = String::from("Ssilka");
    let dlina_ssilki = some(&ssilka);
    println!("{}", dlina_ssilki);
    let mut s1 = String::from("Izmenyaemaya ssilka");
    println!("{}", s1);
    add(&mut s1);
    println!("{}", s1);
    let reference = ret();
    let str_for_slice = String::from("Aboba");
    let slice = &str_for_slice[1..4];
    println!("{}", slice);
    println!("{}",reference);
    println!("What's your name?");
    let mut name = String::new();
    std::io::stdin().read_line(&mut name).expect("Error");
    println!("Your name - {}", name);
}

fn print_value(str1: String){
    println!("{}", str1);
}
fn give() -> String{
    String::from("Give")
}

fn take_and_give(str1:String) -> String{
    str1
}

fn some(s:&String) -> usize{
    s.len()
}
fn add(s:&mut String){
    s.push_str(" Hop dobavil stroky");
}

//Висячая ссылка если пытаться вернуть и передать &String, &string1
fn ret() -> String{
    let string1 = String::from("Ssilka?");
    string1
}