fn main() {
    println!("Hello, world!");
    let a = Counter {};
    let mut h = 0;
    for ele in a {
        println!("{}", ele);
        if ele == h {
            break;
        }
        h += 1;
    }
}
struct Counter {}
impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        Some(3)
    }
}
