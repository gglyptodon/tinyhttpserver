use std::thread;
use std::thread::JoinHandle;

pub struct ThreadPool {
    workers: Vec<Worker>,
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
        let mut workers: Vec<Worker> = Vec::with_capacity(num_threads);
        for i in 0..num_threads {
            workers.push(Worker::new(i));
            //threads.push(thread::JoinHandle<()>)
        }
        ThreadPool { workers }
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
}

impl Worker {
    pub fn new(id: usize) -> Self
    {
        let handle = thread::spawn(|| {});
        Worker {
            id,
            handle
        }
    }
}
