use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
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
fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let a = buf_reader.lines();
    let b = a.map(|result| result.unwrap());
    let c = b.take_while(|line| !line.is_empty());
    let d: Vec<String> = c.collect();
    println!("Request: {:#?}", d);
    let response = "HTTP/1.1 200 OK\r\n\r\n";
    stream.write_all(response.as_bytes()).unwrap();
}
