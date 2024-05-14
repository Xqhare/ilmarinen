use core::fmt;
use std::{error::Error, thread, sync::{mpsc, Arc, Mutex}};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads that populate the pool.
    ///
    /// # Panics
    ///
    /// will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        // This channel is used to communicate between the main thread and the worker threads.
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            // create some threads and store them in the vector
            workers.push(Worker::new(id, Arc::clone(&receiver)))
        }

        ThreadPool { workers, sender: Some(sender),}
    }

    pub fn provision_thread_pool(mint_amount: usize) -> Result<ThreadPool, PoolCreationError> {
        if mint_amount <= 10 {
            ThreadPool::build(2)
        } else if mint_amount <= 50 {
            ThreadPool::build(10)
        } else if mint_amount <= 100 {
            ThreadPool::build(20)
        } else {
            let size = {
                if mint_amount / 5 > 250 {
                    250
                } else {
                    mint_amount / 5
                }
            };
            ThreadPool::build(size)
        }
    }

    pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
       if size < 1 {
            let error_kind: Option<Box<dyn Error>> = Some(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "size can't be less than 1")));
            return Err(PoolCreationError { message: "Pool Creation Failed".to_string(), cause: error_kind})
        } else {
            let output_pool = ThreadPool::new(size);
            Ok(output_pool)
        }
    }

    pub fn execute<F>(&self, f: F) where F: FnOnce() + Send + 'static, 
    {    
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());
        for worker in &mut self.workers {
            // println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, reciever: Arc<Mutex<mpsc::Receiver<Job>>> ) -> Worker {
        // move is capturing all variables from out of scope and move them into the scope
        // of the function.
        let thread = thread::spawn(move || loop {
            let message = reciever.lock().unwrap().recv();

            match message {
                Ok(job) => {
                    //println!("Worker {id} got a job; Worker is executing it.");
                    job();
                }
                Err(_) => {
                    //println!("Worker {id} disconnected; shutting Worker {id} down.");
                    break;
                }
            }
        });
        Worker { id, thread: Some(thread) }
    }
}


#[derive(Debug)]
pub struct PoolCreationError {
    message: String,
    cause: Option<Box<dyn Error>>,
}

impl Error for PoolCreationError {
    fn description(&self) -> &str {
        &self.message
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.cause.as_deref()
    }
}

impl fmt::Display for PoolCreationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = &self.message;
        let cause = &self.cause;
        write!(f, "({}, {:?})", message, cause)
    }
}


