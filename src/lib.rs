use std::{sync::{mpsc, Arc, Mutex}, thread};

pub struct ThreadPool{
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>
}

#[derive(Debug)]
pub enum PoolCreationError {
    InvalidSize
}

type Job = Box<dyn FnOnce() + Send + 'static>;
impl ThreadPool {
    
    /// Create a new thtead pool
    /// size is the number of threads in the pool
    /// 
    /// #Panics
    /// 
    /// The 'new' function will panic if size <= 0
    pub fn new(size: usize) -> ThreadPool {
        assert!(size> 0);

        let (tx, rx) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(rx));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ThreadPool {workers, sender: tx}
    }
    

    /* 
    //build instead of new
    pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size > 0 {
            let mut workers = Vec::with_capacity(size);

            for id in 0..size {
                workers.push(Worker::new(id));
            }
            Ok(ThreadPool{
                workers
            })
        }else {
            Err(PoolCreationError::InvalidSize)
        }
    }
    */
    pub fn execute<F>(&self, f:F) 
    where F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);

        self.sender.send(job).unwrap();
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move|| loop{
            let job = receiver.lock().unwrap().recv().unwrap();

            println!("Worker {id} got a job, executing now...");

            job();
        });
        Worker { id, thread }
    }
}
