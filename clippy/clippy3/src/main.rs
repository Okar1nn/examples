fn main() {
    let my_option: Option<()> = None;
    // if my_option.is_none() {
    //     my_option.unwrap();
    // }
    if my_option.is_some() {}
    // if let Some(_) = my_option {}

    // let my_arr = &[-1, -2, -3 - 4, -5, -6];
    let my_arr = [-1, -2, -3, -4, -5, -6];
    println!("My array! Here it is: {my_arr:?}");

    // let my_empty_vec = vec![1, 2, 3, 4, 5].resize(0, 5);
    let mut my_empty_vec = vec![1, 2, 3, 4, 5];
    my_empty_vec.clear();
    println!("This Vec is empty, see? {my_empty_vec:?}");

    let mut value_a = 45;
    let mut value_b = 66;
    // Let's swap these two!
    // value_a = value_b;
    // value_b = value_a;
    std::mem::swap(&mut value_a, &mut value_b);
    println!("value a: {value_a}; value b: {value_b}");
}
