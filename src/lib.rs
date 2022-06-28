use std::sync::mpsc::Receiver;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;

type Job = Box<dyn FnOnce() + Send + 'static>;
enum Message{
    NewJob(Job),
    Terminate,
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
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
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool{
    fn drop(&mut self) {
        for _ in &mut self.workers{
            println!("Sending shutdown message");
            self.sender.send(Message::Terminate).unwrap();
        }
        for worker in &mut self.workers{
            println!("terminating worker {}", worker.id);
            if let Some(handle) = worker.handle.take(){
                handle.join().unwrap();
            }
        }
    }
}


pub struct Worker {
    id: usize,
    handle: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Message>>>) -> Self {
        let handle = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();
            match message {
                Message::NewJob(j) =>{
                    println!("worker {} got job", id); j();
                },
                Message::Terminate => {
                    println!("worker {} terminating", id);
                    break;
                },
            }
            //let job = receiver.lock().unwrap().recv().unwrap();

        });
        Self { id, handle: Some(handle) }
    }
}
