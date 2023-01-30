struct Point {
    x: i32,
    y: i32,
}

#[allow(path_statements)]
fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    match y {
        Some(ref p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        _ => println!("no match"),
    }
    // y.as_ref().map_or_else(
    //     || println!("No match"),
    //     |p| println!("Coordinates are {}, {}", p.x, p.y),
    // );

    #[allow(clippy::no_effect)]
    y; // Fix without deleting this line.
}
