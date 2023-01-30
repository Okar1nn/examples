fn main() {
    let vec0 = Vec::new();

    let mut vec1 = fill_vec(&vec0);

    // println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);
    println!("vec0 has length {} content `{vec0:?}`", vec0.len());

    vec1.push(88);

    // println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
    println!("vec1 has length {} content `{vec1:?}`", vec1.len());
}

fn fill_vec(vec: &Vec<i32>) -> Vec<i32> {
    let mut vec = vec.clone();

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
