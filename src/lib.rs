use std::{
    thread, 
    sync::{mpsc, Mutex, Arc}};

type Job = Box<dyn FnOnce() + Send + 'static>;
struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        println!("Creating Worker {id}.");
        Worker {
            id,
            thread: thread::spawn(move || {
                let job = receiver.lock().unwrap().recv().unwrap();
                println!("Worker {id} receiver job; executing...");
                job();
                println!("Worker {id} completed execution.");
            }),
        }
    }
}
pub struct ThreadPool {
    worker_threads: Vec<Worker>,
    sender: mpsc::Sender<Job>,
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

        let (sender, receiver) = mpsc::channel();
        
        //wrap receiver in Arc<Mutex<T>> to share between threads since mpsc is multiple producer single consumer
        let receiver = Arc::new(Mutex::new(receiver));

        println!("Setting up {size} workers...");
        let mut worker_threads = Vec::with_capacity(size);        
        for id in 0..size {
            worker_threads.push(Worker::new(id+1, Arc::clone(&receiver)));//create threads
        }
        ThreadPool {worker_threads, sender}
    }

    pub fn execute<T>(&self, f: T) 
    where
        T: FnOnce() + Send + 'static
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
    
}