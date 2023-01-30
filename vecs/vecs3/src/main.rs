// fn main() {
    // 1 СПОСОБ
    // let mut list = vec![1,2,3];
    // list.push(4);
    // list.remove(2);
    // println!("{:?}", list);
    
    // 2 СПОСОБ
    // let mut list:Vec<i64> = Vec::new();
    // list.push(5);
    // list.push(4);
    // list.push(3);
    // list.push(2);
    // list.push(1);
    // println!("{:?}", list);
    
    // 3 СПОСОБ через .get()
    // let list = vec![1,2,3,4,5];
    // match list.get(4){
    //     Some(el) =>{
    //         println!("Element index 4 is {}", el);
    //     }
    //     None => {
    //         println!("Element doesn't exist!");
    //     }
    // }

    // 4 способ через for
    //let list = vec![1,2,3,4,5,6,7];
    // for el in &list {
    //     if el % 2 == 0{
    //         println!("{}", el);
    //     }
    // }
    // for el in list.iter() {
    //     if el % 2 == 0{
    //         println!("{}", el);
    //     }
    // }

//     let list = vec![10,20,30,40];
//     println!("{}",find_avg(&list));

// }
// fn find_avg(vec: &Vec<i32>) -> f64{
//     let mut sum = 0;
//     for v in vec{
//         sum += v
//     }
//     let lenth = (vec.len()) as i32;
//     (sum/lenth) as f64
// } 

enum Types{
    Int(i32),
    Float(f32),
    Bool(bool),
    Text(String)
}
fn main(){
    let v:Vec<Types> = vec![
        Types::Int(5),
        Types::Float(5.5),
        Types::Bool(true),
        Types::Text("Aboba".to_string())
    ];
    match &v[0]{
        Types::Int(int) => {
            println!("Int - {}", int);
        },
        Types::Float(float) => {
            println!("Float - {}", float);
        },
        Types::Bool(bools) =>{
            println!("Bool - {}", bools);
        },
        Types::Text(s) => {
            println!("String - {}", s);
        }
    }
}
