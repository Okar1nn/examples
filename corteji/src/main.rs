fn main() {
    let cortej = (35, 64.5, String::from("Aboba"),true, false);
    println!("{}", cortej.0);
    let (age, sm, name, yes, no) = cortej;
    println!("age ={}, sm = {}, name = {}, yes - {}, no - {}",age,sm,name,yes,no);
}
