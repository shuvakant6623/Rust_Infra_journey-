use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::sync::{Arc, Mutex, Condvar};
use std::thread;

type JobFn = Box<dyn FnOnce() + Send + 'static>;

struct Job {
    priority: usize,
    task: Option<JobFn>,
}

impl Ord for Job {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority.cmp(&other.priority)
    }
}

impl PartialOrd for Job {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Job {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

impl Eq for Job {}

struct State {
    queue: BinaryHeap<Job>,
    shutdown: bool,
}

pub struct ThreadPool {
    workers: Vec<thread::JoinHandle<()>>,
    shared: Arc<(Mutex<State>, Condvar)>,
}

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        assert!(size > 0);

        let state = State {
            queue: BinaryHeap::new(),
            shutdown: false,
        };

        let shared = Arc::new((Mutex::new(state), Condvar::new()));
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            let shared_clone = Arc::clone(&shared);

            let handle = thread::spawn(move || {
                let (lock, cvar) = &*shared_clone;

                loop {
                    let mut state = match lock.lock() {
                        Ok(guard) => guard,
                        Err(poisoned) => {
                            eprintln!("Mutex poisoned in worker {}.", id);
                            poisoned.into_inner()
                        }
                    };

                    while state.queue.is_empty() && !state.shutdown {
                        state = match cvar.wait(state) {
                            Ok(guard) => guard,
                            Err(poisoned) => {
                                eprintln!("Condvar poisoned in worker {}.", id);
                                poisoned.into_inner()
                            }
                        };
                    }

                    if state.shutdown && state.queue.is_empty() {
                        println!("Worker {} exiting.", id);
                        break;
                    }

                    let mut job = match state.queue.pop() {
                        Some(j) => j,
                        None => continue,
                    };

                    drop(state);

                    println!("Worker {} executing priority {}", id, job.priority);

                    if let Some(task) = job.task.take() {
                        task();
                    }
                }
            });

            workers.push(handle);
        }

        ThreadPool { workers, shared }
    }

    pub fn execute<F>(&self, priority: usize, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Job {
            priority,
            task: Some(Box::new(f)),
        };

        let (lock, cvar) = &*self.shared;

        let mut state = match lock.lock() {
            Ok(guard) => guard,
            Err(poisoned) => {
                eprintln!("Mutex poisoned during execute.");
                poisoned.into_inner()
            }
        };

        if state.shutdown {
            eprintln!("ThreadPool is shutting down. Rejecting job.");
            return;
        }

        state.queue.push(job);
        cvar.notify_one();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        let (lock, cvar) = &*self.shared;

        let mut state = match lock.lock() {
            Ok(guard) => guard,
            Err(poisoned) => {
                eprintln!("Mutex poisoned during shutdown.");
                poisoned.into_inner()
            }
        };

        state.shutdown = true;

        cvar.notify_all();
        drop(state);

        for worker in self.workers.drain(..) {
            if let Err(e) = worker.join() {
                eprintln!("Worker panicked: {:?}", e);
            }
        }

        println!("ThreadPool shutdown complete.");
    }
}