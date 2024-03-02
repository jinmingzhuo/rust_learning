fn main() {
    println!("Hello, world!");
    let a = Point { x: 1, y: 43 };
    a.my_print();
}
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}
impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}
trait MyPrint: std::fmt::Display {
    fn my_print(&self) {
        println!("********");
        println!("{}", self);
        println!("********");
    }
}
impl MyPrint for Point {}
