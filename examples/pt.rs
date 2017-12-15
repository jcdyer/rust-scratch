
#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

fn move_to(&Point { ref x, ref y }: &Point, dx: f64, dy: f64) -> Point {
    Point { x: x + dx, y: y + dy }
}

fn main() {
    let pt = Point { x: 4.3, y: 8.1 };
    println!("{:?}, {:?}", pt, move_to(&pt, 3.0, 2.0));
}
