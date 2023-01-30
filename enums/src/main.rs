// enum Person {
//     Adult, 
//     Underage
// }
// fn main() {
//     let person = Person::Adult;
//     match person {
//         Person::Adult => {
//             println!("Тебе можно!");
//             println!("Ты взрослый!");
//         },
//         Person::Underage => {
//             println!("Тебе нельзя!");
//             println!("Ты маленький!");
//         }
//     }
// }

// enum Say{
//     Hi(String),
//     Bye(String),
//     Gm(String),
//     Gn(String)
// }
// fn main(){
//     let say = Say::Gm("good morning".to_string());
//     match say {
//         Say::Hi(h) => println!("{}", h),
//         Say::Bye(b) => println!("{}", b),
//         Say::Gm(gm) => println!("{}", gm),
//         Say::Gn(gn) => println!("{}", gn),
//     }
// }

// enum MathOperation{
//     Add(f64,f64),
//     Sub(f64,f64),
//     Mul(f64,f64),
//     Div(f64,f64)
// }
// impl MathOperation {
//     fn math_operations(&self) -> f64{
//         match self{
//             &MathOperation::Add(a,b) => a + b,
//             &MathOperation::Sub(a,b) => a - b,
//             &MathOperation::Mul(a,b) => a * b,
//             &MathOperation::Div(a,b) => a / b,
//         }
//     }
// }
// fn main(){
//     let operation = MathOperation::Mul(18.0,9.0);
//     let result = operation.math_operations();
//     println!("{}", result);
    
// }
// enum WebEvent {
//     PageLoad,
//     PageUnLoad,
//     KeyChar(char),
//     KeyMsg(String),
//     Click{x:u32, y:u32}
// }
// impl WebEvent{
// fn event_exp(&self){
//     match self{
//         Self::PageLoad => println!("Страница загружена"),
//         Self::PageUnLoad => println!("Страница выгружена"),
//         Self::KeyChar(c) => println!("нажат {} символ", c),
//         Self::KeyMsg(s) => println!("Написана вот эта {} строчка", s),
//         Self::Click{x, y} => println!("Нажато по вот этим координатам {}, {}", x, y)
//     }
// }
// }
// fn main(){
//     let pageload = WebEvent::PageLoad;
//     let pageunload = WebEvent::PageUnLoad;
//     let keychar = WebEvent::KeyChar('s');
//     let keymsg = WebEvent::KeyMsg("Aboba".to_string());
//     let clickk = WebEvent::Click{x:5, y:6};
//     pageload.event_exp();
//     pageunload.event_exp();
//     keychar.event_exp();
//     keymsg.event_exp();
//     clickk.event_exp();
//     // event_exp(pageload);
//     // event_exp(pageunload);
//     // event_exp(keychar);
//     // event_exp(keymsg);
//     // event_exp(clickk);
// }
// fn main() {
//     let strr="Hello";
//     if let Some(a) = strr.get(0..1){
//         println!("{:?}", a);
//     }
// }
// fn main(){
//     let name = "Rello".to_string();
//     if name.get(0..1) == Some("R"){
//         println!("Aboba")
//     }
// }
// fn are_you_playing_banjo(name: &str) -> String {
//     match &name[0..1] {
//         "R" | "r" => format!("{} plays banjo", name),
//         _ => format!("{} does not play banjo", name)
//     }
// }
