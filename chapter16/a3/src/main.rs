use std::{
    cell::RefCell,
    str::FromStr,
    sync::{mpsc, Arc, Barrier},
    thread,
    time::Duration,
};

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("hi1"),
            String::from("hi2"),
            String::from("from"),
        ];
        for ele in vals {
            tx.send(ele).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("hi1"),
            String::from("hi2"),
            String::from("from"),
        ];
        for ele in vals {
            tx1.send(ele).unwrap();
            thread::sleep(Duration::from_secs(2));
        }
    });
    for ele in rx.iter() {
        println!("Got: {}", ele);
    }
    println!("{:?}", rx);
}
