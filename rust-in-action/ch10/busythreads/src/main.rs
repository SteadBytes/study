//! Demonstration of the effect of spawning *many* threads and comparing the
//! time taken wating for threads to consume 100% CPU in a spin loop for 20ms
//! vs a 20ms thread sleep.
//!
//! For the first few 'batches', the spin loop method will be closer to the
//! 20ms target as OS thread sleep isn't perfectly accurate. From the
//! [`std::thread::sleep` docs](https://doc.rust-lang.org/std/thread/fn.sleep.html):
//!
//! > Puts the current thread to sleep for at least the specified amount of time.
//! > The thread may sleep longer than the duration specified due to scheduling
//! > specifics or platform-dependent functionality. It will never sleep less.
//!
//! A spin loop is therefore more appropriate for applications that are very
//! timing sensitive or need to pause a thread for short periods of time.
//!
//! CPU-intensive multi-threading scales poorly past the number of physical
//! cores on the CPU. This is demonstrated here by the measured times for each
//! thread to complete in the spin loop method (which consumes ~100% CPU)
//! increasing greatly as the thread count increases.
use std::{thread, time};

const THREAD_PAUSE: time::Duration = time::Duration::from_millis(20);
const MAX_THREADS: usize = 1001;

fn spawn_threads<F>(f: F)
where
    F: Fn() -> (),
    F: 'static + Send + Copy,
{
    for n in 1..MAX_THREADS {
        let mut handlers: Vec<thread::JoinHandle<()>> = Vec::with_capacity(n);

        let start = time::Instant::now();
        for _m in 0..n {
            let handle = thread::spawn(f);
            handlers.push(handle);
        }

        for handle in handlers {
            handle.join().unwrap();
        }

        let finish = time::Instant::now();
        println!("{}\t{:02?}", n, finish.duration_since(start));
    }
}

fn main() {
    println!("Thread sleep:");

    spawn_threads(|| {
        thread::sleep(THREAD_PAUSE);
    });

    println!("Spin loop:");

    spawn_threads(|| {
        let start = time::Instant::now();
        while start.elapsed() < THREAD_PAUSE {
            // Signal to the OS that this thread can be de-scheduled
            // Alternative is std::sync::atomic::spin_loop_hint(), the CPU
            // *might* use the hint to to turn off functionality and save CPU.
            // Not all CPUs have this instruction - noop in that case.
            thread::yield_now();
        }
    });
}
