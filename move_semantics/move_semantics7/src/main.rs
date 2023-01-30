use std::io;
fn main(){
 let mut a_str = String::new();
 let mut b_str = String::new();
 let mut c_str = String::new();
 loop {
     
 println!("Решить квадратное уравнение:");

 println!("Введите первый член:");
 match io::stdin().read_line(&mut a_str) {
     Ok(_) => {},
     Err(e) => println!("Ошибка! {}", e),
 }
 println!("Введите второй член:");
 match io::stdin().read_line(&mut b_str) {
     Ok(_) => {},
     Err(e) => println!("Ошибка! {}", e),
 }
 println!("Введите третий член:");
 match io::stdin().read_line(&mut c_str) {
     Ok(_) => {},
     Err(e) => println!("Ошибка! {}", e),
 }
 
 let a:f64 = a_str.trim().parse().unwrap();
 let b:f64 = b_str.trim().parse().unwrap();
 let c:f64 = c_str.trim().parse().unwrap();

 let d:f64 = (b * b) - 4.0 * (a * c);
 if d > 0.0 {
    let x1 = ((-b) + d.sqrt()) / (2.0 * a);
    let x2 = ((-b) - d.sqrt()) / (2.0 * a);
    println!("Решено!\n есть 2 корня\n D = {}\n Корень 1 = {}\n Корень 2 = {}\n", d, x1,x2);
 }
 if d == 0.0 {
    let x = (-b) / (2.0 * a);
    println!("Решено!\n D = {}\nКорень равен = {}", d, x);
 }
 if d < 0.0{
    println!("Корней не существует!\n D < 0\n D = {}", d);
 }
}
}