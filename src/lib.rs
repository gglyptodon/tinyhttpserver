use std::alloc::handle_alloc_error;
use std::sync::mpsc::Receiver;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;

type Job = Box<dyn FnOnce() + Send + 'static>;

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
    pub fn execute<F>(&self, job_fn: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(job_fn);
        self.sender.send(job).unwrap();
    }
}

pub struct Worker {
    id: usize,
    handle: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Self {
        let handle = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();
            println!("worker {} got job", id);
            job();
        });
        Self { id, handle }
    }
}
