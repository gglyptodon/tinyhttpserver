use std::sync::{Arc, mpsc, Mutex};
use std::sync::mpsc::Receiver;
use std::thread;
use std::thread::JoinHandle;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    /// Create new Threadpool.
    /// # Panics
    /// new panics when ``num_threads`` is smaller than 1
    /// # Example
    /// ```should_panic
    /// use tinyhttpserver::ThreadPool;
    /// let tp = ThreadPool::new(0); // panics
    /// ```
    pub fn new(num_threads: usize) -> Self {
        assert!(num_threads > 0);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers: Vec<Worker> = Vec::with_capacity(num_threads);
        for i in 0..num_threads {
            workers.push(Worker::new(i, Arc::clone(&receiver)));
        }
        ThreadPool { workers, sender }
    }
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
}

pub struct Worker {
    id: usize,
    handle: thread::JoinHandle<()>,
    //receiver: mpsc::Receiver<Job>
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Self {
        let handle = thread::spawn(|| {
            receiver;
        });
        Worker { id, handle }
    }
}

struct Job;
