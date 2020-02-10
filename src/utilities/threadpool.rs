use log::{info, warn};
use std::sync::{Arc, Mutex};
use std::thread;

//https://doc.rust-lang.org/book/ch20-02-multithreaded.html

enum Message {
    NewJob(Job),
    Terminate,
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: crossbeam::Sender<Message>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = crossbeam::unbounded();
        // To share ownership across multiple threads and allow the threads
        // to mutate the value, we need to use Arc<Mutex<T>>.
        // The Arc type will let multiple workers own the receiver,
        // and Mutex will ensure that only one worker gets a job from the receiver at a time.
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            //create some threads and store them in the vector
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        warn!("Sending terminate message to all workers.");

        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        warn!("Shutting down all workers.");
        for worker in &mut self.workers {
            //info!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

/*
1. Define a Worker struct that holds an id and a JoinHandle<()>.
2. Change ThreadPool to hold a vector of Worker instances.
3. Define a Worker::new function that takes an id number and returns a Worker instance that holds the id and a thread spawned with an empty closure.
4. In ThreadPool::new, use the for loop counter to generate an id, create a new Worker with that id, and store the worker in the vector.
*/

struct Worker {
    id: usize,
    // Each thread should return a Tile
    // thread : thread::JoinHandle<Arc<Mutex<crossbeam::Receiver<Job>>>>
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    // we need the closure to loop forever,
    // asking the receiving end of the channel for a job
    // and running the job when it gets one
    fn new(id: usize, receiver: Arc<Mutex<crossbeam::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();
            match message {
                Message::NewJob(job) => {
                    info!("Worker {} got a job, executing.", id);
                    job();
                }

                Message::Terminate => {
                    info!("Worker {} was told to terminate", id);
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}
