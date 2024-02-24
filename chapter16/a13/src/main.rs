use std::rc::Rc;

fn main() {
    println!("Hello, world!");
    let v = Rc::new(5);
    let t = std::thread::spawn(move || {
        println!("{}", v);
    });
    t.join().unwrap();
}
