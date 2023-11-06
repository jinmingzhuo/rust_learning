trait Draw {
    fn draw(&mut self);
}
struct Button {
    id: u32,
}
impl Draw for Button {
    fn draw(&mut self) {
        println!("this is the id of botton: {}", self.id);
    }
}
struct Select {
    id: u32,
}
impl Draw for Select {
    fn draw(&mut self) {
        println!("select id {}", self.id);
    }
}
fn main() {
    let b = Button { id: 11 };
    let s = Select { id: 12 };
    let bb = Box::new(b);
    let bs = Box::new(s);
    let v: Vec<Box<dyn Draw>> = vec![bb, bs];
    for mut ele in v {
        ele.draw();
    }
    println!("{}", get_static_str());
    let mut a: [i32; 4] = [1, 2, 3, 4];
    for ele in a.iter_mut() {
        *ele = 999;
        let h: *mut i32 = ele;
        unsafe {
            *h = 88;
        }
        println!("{:?}", h);
    }
    println!("{:?}", a);
}
fn get_static_str() -> &'static str {
    let mut s = String::new();
    s.push_str("aaa");

    // let a = Box::leak(Box::new(s));
    let a = Box::leak(s.into_boxed_str());
    a
}
/* Make it work with const generics */
fn my_function<const N: usize>() -> [u32; N] {
    [123; N]
}
