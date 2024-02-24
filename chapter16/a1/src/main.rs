// use core::asserting::Printable;
use std::{thread, time::Duration};

fn main() {
    println!("Hello, world!");
    let t1 = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1000..1005 {
        println!("this is the main thread's output {}", i);
        thread::sleep(Duration::from_millis(2));
    }
    let a = t1.join();
    a.unwrap();
}
