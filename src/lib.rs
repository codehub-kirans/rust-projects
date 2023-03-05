//! #mini-web-server
//!
//! `mini-web-server` is a simple HTTP web server that uses a thread pool to respond asynchronously.
//! Supports 2 arguments - threads(no of threads) and port(port number). Default threads is 4 and default port is 7878

use std::{
    error::Error,
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    sync::{mpsc, Arc, Mutex},
    thread,
    time::Duration,
};

const THREAD_SIZE: usize = 4;
const PORT: usize = 7878;

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct Config {
    pub thread_size: usize,
    pub port: usize,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let thread_size = match args.next() {
            Some(arg) => {
                if let Ok(arg) = arg.parse() {
                    arg
                } else {
                    return Err("Please type a number for number of worker threads.");
                }
            }
            None => {
                println!(
                    "No thread pool size specified. Using default {THREAD_SIZE} worker threads."
                );
                THREAD_SIZE
            }
        };

        let port = match args.next() {
            Some(arg) => {
                if let Ok(arg) = arg.parse() {
                    arg
                } else {
                    return Err("Please type a number for the TCP listening port higher than 1023. Default port is 7878.");
                }
            }
            None => {
                println!("No port number specified. Using default {PORT} worker threads.");
                PORT
            }
        };

        Ok(Config { thread_size, port })
    }
}

struct Worker {
    _id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        println!("Creating Worker {id}.");

        // create a thread that continuously loops checking the channel for jobs
        let thread = thread::spawn(move || loop {
            let msg = receiver.lock().unwrap().recv();
            match msg {
                Ok(job) => {
                    println!("Worker {id} receiver job; executing...");
                    job();
                }
                Err(_e) => {
                    println!("Worker {id} received disconnection request. Shutting down");
                    break;
                }
            }

            println!("Worker {id} completed execution.");
        });

        Worker {
            _id: id,
            thread: Some(thread),
        }
    }
}
pub struct ThreadPool {
    worker_threads: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    /// Todo: return Result<ThreadPool, PoolCreationError>
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        // create a mpsc channel to send closure function implmenting work to be executed by spawned thread
        let (sender, receiver) = mpsc::channel();

        //wrap receiver in Arc<Mutex<T>> to share between threads since mpsc is multiple producer single consumer
        let receiver = Arc::new(Mutex::new(receiver));

        println!("Setting up {size} workers...");
        let mut worker_threads = Vec::with_capacity(size);
        for id in 0..size {
            worker_threads.push(Worker::new(id + 1, Arc::clone(&receiver))); //create threads
        }
        ThreadPool {
            worker_threads,
            sender: Some(sender),
        }
    }

    pub fn execute<T>(&self, f: T)
    where
        T: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        //close the channel by dropping the sender end
        //so that the receiver end receives error to exit the worker thread's loop
        drop(self.sender.take());

        //shut down worker threads by calling join.
        for worker in &mut self.worker_threads {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    //listen for tcp connections with TcpListner and bind to a port
    let address = format!("127.0.0.1:{}", config.port);
    let listener = TcpListener::bind(address)?;
    let thread_pool = ThreadPool::new(config.thread_size);

    //iterate through sequence of streams
    for stream in listener.incoming() {
        let stream = stream?;

        thread_pool.execute(|| {
            handle_connection(stream);
        });
    }

    Ok(())
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
