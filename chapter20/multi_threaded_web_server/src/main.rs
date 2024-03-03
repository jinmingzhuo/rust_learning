use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    thread::{self, JoinHandle},
    time::Duration,
};

use multi_threaded_web_server::*;
fn main() {
    println!("Hello, world!");
    let pool = ThreadPool::new(100);
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for ele in listener.incoming() {
        let stream = ele.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
    }
    println!("shutdown!!!!!!!!!!");
}
fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let a = buf_reader.lines().next().unwrap().unwrap();
    // println!("Request: {:#?}", a);
    let b = a.as_str();

    let (status_line, file_uri) = match b {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };
    let contents = fs::read_to_string(file_uri).unwrap();
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
}
