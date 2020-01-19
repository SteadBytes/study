use std::error::Error;
use std::fmt;
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
}

impl ThreadPool {
    ///  Create a new `ThreadPool`.
    ///
    /// `size` is the number of threads (`> 0`) in the pool
    pub fn new(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size == 0 {
            return Err(PoolCreationError);
        }

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id));
        }

        Ok(ThreadPool { workers })
    }

    pub fn execute<F>(&self, f: F)
    where
        // Closure to be exeucted once, in another thread
        F: FnOnce() + Send + 'static,
    {
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
    fn new(id: usize) -> Worker {
        let thread = thread::spawn(|| {});

        Worker { id, thread }
    }
}
