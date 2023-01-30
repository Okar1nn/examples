fn main() {
    // let vec0 = Vec::new();

    // let mut vec1 = fill_vec(vec0);
    let mut vec1 = fill_vec();

    // println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
    println!("vec1 has length {} content `{vec1:?}`", vec1.len());

    vec1.push(88);

    // println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
    println!("vec1 has length {} content `{vec1:?}`", vec1.len());
}

fn fill_vec() -> Vec<i32> {
    // let mut vec = vec;
    let mut vec = Vec::new();

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec

    // vec![22, 44, 66]
}
