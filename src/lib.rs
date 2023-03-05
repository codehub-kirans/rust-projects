use std::{ thread, sync::{ mpsc, Mutex, Arc } };

type Job = Box<dyn FnOnce() + Send + 'static>;
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
                },
                Err(e) => {
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
        ThreadPool { worker_threads, sender: Some(sender) }
    }

    pub fn execute<T>(&self, f: T) where T: FnOnce() + Send + 'static {
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