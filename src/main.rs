// source: https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html

use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread, 
    time::Duration,
};

use tutorial_simple_webserver::ThreadPool;

fn main() {
    let port: i32 = 7878;
    let listener: TcpListener = TcpListener::bind(
        format!("127.0.0.1:{}", port)).unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream: TcpStream = stream.unwrap();
        
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = match request_line.as_str() {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "sleep.html")
        },
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!(
        "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
    );

    stream.write_all(response.as_bytes()).unwrap(); //ToDo: Error handling
}