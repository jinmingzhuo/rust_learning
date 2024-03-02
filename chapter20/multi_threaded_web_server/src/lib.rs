use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    thread::{self, JoinHandle},
    time::Duration,
};

pub struct ThreadPool {
    workers: Vec<Worker>,
}
struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let mut workers = Vec::with_capacity(size);
        for i in 0..size {
            workers.push(Worker::new(i));
        }
        ThreadPool { workers }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce(),
        F: Send + 'static,
    {
    }
}
impl Worker {
    pub fn new(id: usize) -> Self {
        let thread = thread::spawn(|| {});
        Worker { id, thread }
    }
}
