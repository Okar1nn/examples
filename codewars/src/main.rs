// fn main() {
//     let year = 1705;
//     let cent= year + 99;
//     let string = cent.to_string();
//     let two_symbols = string.get(0..2).expect("Wtf");
//     let to_u32:u32 = two_symbols.parse().unwrap();
//     println!("{}",to_u32);
// }
// fn main(){
//     let year = 1705;
//     let x = (year - 1) / 100 + 1;
//     println!("{}", x);
// }
fn poschitay(s1:u32,s2:u32,s3:u32) -> char{
    let ave_sum = (s1 + s2 + s3)/3;
        match ave_sum{
        90..=u32::MAX => 'A',
        80..=90 => 'B',
        70..=80 => 'C',
        60..=70 => 'D',
        0..=60 => 'F',
    }
}
fn main(){
    let s1 = 90;
    let s2 = 93;
    let s3 = 95;
    let x = poschitay(s1,s2,s3);
    println!("{}", x);
}
// fn main(){
//     let x = 50;
//     match x{
//         0..=50 => println!("Aboba"),
//         _=> println!("No aboba"),
//     }
// }