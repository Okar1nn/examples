fn main() {
    let radius = 5.0;

    // let area = f32::consts::PI * f32::powi(radius, 2);
    // let area = f32::consts::PI * radius.powi(2);
    let area = f32::consts::PI * radius * radius;

    println!("The area of a circle with radius {radius:.2} is {area:.5}!")

}
