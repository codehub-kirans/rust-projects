use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use simple_web_server::ThreadPool;

fn main() {
    //listen for tcp connections with TcpListner and bind to a port
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let thread_pool = ThreadPool::new(4);

    //iterate through sequence of streams
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        thread_pool.execute(|| {
            handle_connection(stream);
        })
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let mut http_request = buf_reader.lines();
    let http_request_line = http_request.next().unwrap().unwrap();

    //handle routes
    let (status_line, file_name) = match &http_request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK\r\n", "welcome.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK\r\n", "welcome.html")
        }
        _ => ("HTTP/1.1 400 NOT FOUND\r\n", "error.html"),
    };

    //print HTTP Request to console
    let http_request: Vec<_> = http_request
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    println!("Connection Established. HTTP Req => {http_request_line}\n{http_request:#?}");

    let contents = fs::read_to_string(file_name).unwrap();
    let content_length = contents.len();
    let response = format!("{status_line}Content-Length: {content_length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
