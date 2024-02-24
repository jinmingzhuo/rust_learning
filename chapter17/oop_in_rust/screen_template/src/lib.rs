pub fn add(left: usize, right: usize) -> usize {
    left + right
}
pub trait Draw {
    fn draw(&self);
}
#[derive(Debug)]
pub struct Screen<T: Draw + std::fmt::Debug> {
    pub components: Vec<Box<T>>,
}
impl<T: Draw + std::fmt::Debug> Screen<T> {
    fn run(&self) {
        for ele in &self.components {
            ele.draw()
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
        let a = Screen { components: vec![] };
        println!("{:?}", a);
    }
}
