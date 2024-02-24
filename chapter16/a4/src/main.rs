use std::{
    sync::mpsc,
    thread::{self, Thread},
    time::Duration,
};

#[derive(Debug)]
enum Jmz {
    A(i32),
    B(String),
}
fn main() {
    println!("Hello, world!");
    let (rx, tx) = mpsc::sync_channel(5);

    let mut list = Vec::new();
    for i in 0..6 {
        let r = rx.clone();
        let a = thread::spawn(move || {
            for j in 1..3 {
                match i {
                    5 => r.send(Jmz::B(String::from("---------------"))).unwrap(),
                    _ => r.send(Jmz::A(i)).unwrap(),
                }
                println!("thread {} send over!", i);
                // r.send(Jmz::A(i)).unwrap();
                thread::sleep(Duration::from_secs(10));
            }
            // println!("thread {} send over!", i);
        });
        list.push(a);
    }
    let aa = thread::spawn(move || {
        thread::sleep(Duration::from_secs(5));
        for ele in tx {
            println!("{:?}", ele);
        }
    });
    for ele in list {
        ele.join().unwrap();
    }
    // aa.join().unwrap();
}
