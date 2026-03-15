use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::sync::{Arc, Mutex, Condvar};
use std::thread;
use std::time::Duration;
use std::panic::{self, AssertUnwindSafe};

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
    capacity: usize,
}

pub struct ThreadPool {
    workers: Vec<thread::JoinHandle<()>>,
    shared: Arc<(Mutex<State>, Condvar)>,
}

impl ThreadPool {
    pub fn new(workers: usize, capacity: usize) -> Self {
        assert!(workers > 0);
        assert!(capacity > 0);

        let state = State {
            queue: BinaryHeap::new(),
            shutdown: false,
            capacity,
        };

        let shared = Arc::new((Mutex::new(state), Condvar::new()));
        let mut handles = Vec::with_capacity(workers);

        for id in 0..workers {
            let shared_clone = Arc::clone(&shared);

            let handle = thread::spawn(move || {
                let (lock, cvar) = &*shared_clone;

                loop {
                    let mut state = lock.lock().unwrap();

                    while state.queue.is_empty() && !state.shutdown {
                        let (guard, timeout) =
                            cvar.wait_timeout(state, Duration::from_secs(2)).unwrap();

                        state = guard;

                        if timeout.timed_out() {
                            println!("Worker {} idle", id);
                        }
                    }

                    if state.shutdown && state.queue.is_empty() {
                        println!("Worker {} exiting", id);
                        break;
                    }

                    let mut job = match state.queue.pop() {
                        Some(j) => j,
                        None => continue,
                    };

                    cvar.notify_one();
                    drop(state);

                    println!("Worker {} executing priority {}", id, job.priority);

                    if let Some(task) = job.task.take() {

                        let result = panic::catch_unwind(AssertUnwindSafe(task));

                        if result.is_err() {
                            println!("Worker {} caught panic while executing job", id);
                        }
                    }
                }
            });

            handles.push(handle);
        }

        ThreadPool {
            workers: handles,
            shared,
        }
    }

    pub fn execute<F>(&self, priority: usize, job: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Job {
            priority,
            task: Some(Box::new(job)),
        };

        let (lock, cvar) = &*self.shared;

        let mut state = lock.lock().unwrap();

        while state.queue.len() >= state.capacity && !state.shutdown {
            state = cvar.wait(state).unwrap();
        }

        if state.shutdown {
            return;
        }

        state.queue.push(job);
        cvar.notify_one();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        let (lock, cvar) = &*self.shared;

        let mut state = lock.lock().unwrap();
        state.shutdown = true;

        cvar.notify_all();
        drop(state);

        for worker in self.workers.drain(..) {
            if let Err(e) = worker.join() {
                println!("Worker panicked: {:?}", e);
            }
        }

        println!("ThreadPool shutdown complete.");
    }
}