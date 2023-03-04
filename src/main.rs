use std::{
    fs,
    net::{TcpListener, TcpStream},
    io::{prelude::*, BufReader},
};

fn main() {
    //listen for tcp connections with TcpListner and bind to a port
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    
    //iterate through sequence of streams
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream){
    print!("Connection Established. HTTP Req => ");

    let buf_reader = BufReader::new(&stream);
    let mut http_request = buf_reader.lines();
    let http_request_line = http_request.next().unwrap().unwrap();

    let (status_line, file_name) = if http_request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK\r\n", "welcome.html")
    } else {
        ("HTTP/1.1 400 NOT FOUND\r\n", "error.html")
    };

    let http_request: Vec<_> = http_request
    .map(|result| result.unwrap())
    .take_while(|line| !line.is_empty())
    .collect();
    println!("{}\n{:#?}", http_request_line, http_request);

    let contents = fs::read_to_string(file_name).unwrap();
    let content_length = contents.len();
    let response = format!("{status_line}Content-Length: {content_length}\r\n\r\n{contents}");
    
    stream.write_all(response.as_bytes()).unwrap();
}
