use std::{sync::{mpsc, Arc, Mutex}, thread};

pub struct ThreadPool {
    // The thread is not meant to return anything so we use a ()
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Create a new ThreadPool
    /// 
    /// The size is the number of threads in the pool.
    /// 
    /// # Panics
    /// 
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        // The receiver needs to have shared ownership between the threads and it needs to be thread safe mutable
        // Shared ownership -> smart pointers
        // We need the receiver to be thread safe -> Arc smart pointer
        // We need it to be mutable -> Mutex
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    // Where includes traits of the generic type F
    where
        F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        // The closure may outlive the current function but it borrows receiver which is owned by the current function
        // Thus the move keyword is used to move the receiver into the closure
        // Finally, the closure needs to keep looking for jobs once it is completed so we use the loop keyword
        let thread = thread::spawn(move || loop {
            let job =
            receiver
            .lock()
            .unwrap()
            .recv()
            .unwrap();
            println!("Worker {} got a job; executing.", id);

            job();
        });
        Worker { id, thread }
    }
}

