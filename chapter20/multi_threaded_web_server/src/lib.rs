use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    sync::mpsc::{self, Receiver},
    sync::Arc,
    sync::Mutex,
    thread::{self, JoinHandle},
    time::Duration,
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}
// struct Job;
type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let mut workers = Vec::with_capacity(size);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        for i in 0..size {
            workers.push(Worker::new(i, receiver.clone()));
        }
        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce(),
        F: Send + 'static,
    {
        let job = Box::new(f);
        // self.sender.send(job).unwrap();
        if let Some(sender) = &self.sender {
            sender.send(job).unwrap();
        }
    }
}
impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());
        for ele in &mut self.workers {
            println!("Shutting down worker {}", ele.id);
            if let Some(a) = ele.thread.take() {
                a.join().unwrap();
            }
        }
    }
}
impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Self {
        let thread = thread::spawn(move || loop {
            let a = receiver
                .lock()
                .expect(format!("Worker {} error", id).as_str());
            let recv = a.recv();
            // match a.recv(){
            match recv {
                Ok(b) => {
                    println!("Worker {id} got a job; executing.");
                    b();
                }
                Err(e) => {
                    println!("Worker {} sender is closed", id);
                    break;
                }
            }
        });
        Worker {
            id,
            thread: Some(thread),
        }
    }
}
