fn main() {
    let array = [1,2,2,3,4,5,6,6,7,8,9,9];
    println!("{:?}", array);
    let mut i = 0;
    while i < array.len(){
        let mut j = i + 1;
        while j < array.len(){
            if array[i] == array[j]{
            println!("{}", array[i]);
            }
            j+= 1;
        }
        i += 1;
    }
}
