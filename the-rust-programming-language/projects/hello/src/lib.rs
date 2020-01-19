use std::error::Error;
use std::fmt;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

#[derive(Debug)]
pub struct PoolCreationError;

impl Error for PoolCreationError {
    fn description(&self) -> &str {
        "ThreadPool size must be > 0"
    }
}

impl fmt::Display for PoolCreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PoolCreationError: {}", self.description())
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

/// Workaround to enable calling a boxed `FnOnce` closure by taking ownership
/// of the value inside the `Box<T>` using `self: Box<Self>` and then calling
/// it.
///
/// This won't compile:
///
/// ```
/// let f: Box<dyn FnOnce()> = Box::new(|| println!("Hello from the closure"));
/// (*job)()
/// ```
///
/// Rust won't allow a value to be moved outside of a `Box<T>` as, by
/// definition, it does not know the size of the value inside. However, it does
/// allow a method to take ownership of a `Self` value stored in a `Box<T>`.
trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    /// Call a boxed closure.
    ///
    /// The *method* takes ownership of the closure after moving it out of the
    /// box and can then call it.
    fn call_box(self: Box<F>) {
        // Rust allows a method to take ownership of a `Self` value that is
        // inside a `Box<T>`
        (*self)()
    }
}

/// Holds closures to be executed in worker threads
type Job = Box<dyn FnBox + Send + 'static>;

impl ThreadPool {
    ///  Create a new `ThreadPool`.
    ///
    /// `size` is the number of threads (`> 0`) in the pool
    pub fn new(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size == 0 {
            return Err(PoolCreationError);
        }

        let (sender, receiver) = mpsc::channel();

        // Each thread needs mutable access to `receiver` in order to take jobs
        // from the queue and workers should not take jobs simultaneously
        // `Arc` allows for multiple owners, `Mutex` ensures only one worker
        // gets a job at a time
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        Ok(ThreadPool { workers, sender })
    }

    pub fn execute<F>(&self, f: F)
    where
        // Closure to be exeucted once, in another thread
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(job).unwrap();
    }
}

/// `ThreadPool` worker thread.
///
/// External code doesn't need to know that `ThreadPool` internally uses these
/// `Worker`s to implement threading functionality -> keep private.
struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver
                .lock()
                // Mutex may be poisoned by another worker panicking without
                // releasing the lock
                .unwrap()
                .recv()
                // Sending side of the channel may be closed
                .unwrap();
            println!("Worker {} got a job; executing.", id);
            job.call_box();
        });

        Worker { id, thread }
    }
}
