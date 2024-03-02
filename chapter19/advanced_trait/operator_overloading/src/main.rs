use std::ops::Add;
fn main() {
    println!("Hello, world!");
    let a = Point { x: 1, y: 1 };
    let b = Point { x: 1, y: 1 };
    // println!("{:?}", a + b);
    println!("{:?}", a + b + 2);
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: i32,
    y: i32,
}
impl Add for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl Add<i32> for Point {
    type Output = Self;
    fn add(self, rhs: i32) -> Self::Output {
        Point {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}
