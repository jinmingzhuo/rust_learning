use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};
fn main() {
    println!("Hello, world!");
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!(
            ">>>>>>>>>>>>>>>>>>>>>>>>>{:?}<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<",
            stream
        );
        handle_connection(stream);
    }
}
fn handle_connection1(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let a = buf_reader.lines();
    let b = a.map(|result| result.unwrap());
    let c = b.take_while(|line| !line.is_empty());
    let d: Vec<String> = c.collect();
    println!("Request: {:#?}", d);
    let status_line = "HTTP/1.1 200 OK";

    let content = fs::read_to_string("./hello.html").unwrap();
    let content_len = content.len();
    let response = format!("{status_line}\r\nContent-Length: {content_len}\r\n\r\n{content}");
    stream.write_all(response.as_bytes()).unwrap();
}
fn handle_connection2(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let a = buf_reader.lines().next().unwrap().unwrap();
    println!("Request: {:#?}", a);
    if a == "GET / HTTP/1.1" {
        let status_line = "HTTP/1.1 200 OK";

        let content = fs::read_to_string("./hello.html").unwrap();
        let content_len = content.len();
        let response = format!("{status_line}\r\nContent-Length: {content_len}\r\n\r\n{content}");
        stream.write_all(response.as_bytes()).unwrap();
    } else {
        eprintln!("unkown url");
        let status_line = "HTTP/1.1 404 NOT FOUND";
        let contents = fs::read_to_string("./404.html").unwrap();
        let length = contents.len();
        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
        stream.write_all(response.as_bytes()).unwrap();
    }
}
fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let a = buf_reader.lines().next().unwrap().unwrap();
    println!("Request: {:#?}", a);
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
