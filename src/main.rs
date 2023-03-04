use std::{
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

fn handle_connection(stream: TcpStream){
    println!("Connection Established. HTTP Req =>");

    let buf_reader = BufReader::new(stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("{:#?}", http_request);
}
