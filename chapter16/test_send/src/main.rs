use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

#[derive(Debug)]
struct MyBox(*const u8);
unsafe impl Send for MyBox {}
unsafe impl Sync for MyBox {}

fn main() {
    let b = &MyBox(5 as *const u8);
    let v = Arc::new(Mutex::new(b));
    let t = thread::spawn(move || {
        let _v1 = v.lock().unwrap();
    });
    let p = 0x00000000004016f0 as *const i64;
    unsafe {
        let n = *p;
        println!("{:x}", n);
    }

    t.join().unwrap();
}
