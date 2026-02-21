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

pub struct ThreadPool {
    workers: Vec<Worker>,
    shared: Arc<(Mutex<BinaryHeap<Job>>, Condvar)>,
}

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        assert!(size > 0);

        let shared = Arc::new((Mutex::new(BinaryHeap::new()), Condvar::new()));
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&shared)));
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
        let mut queue = lock.lock().unwrap();
        queue.push(job);

        // Wake up one worker
        cvar.notify_one();
    }
}

struct Worker {
    _id: usize,
}

impl Worker {
    fn new(id: usize, shared: Arc<(Mutex<BinaryHeap<Job>>, Condvar)>) -> Self {
        thread::spawn(move || {
            let (lock, cvar) = &*shared;

            loop {
                let mut queue = lock.lock().unwrap();

                // Wait until queue has jobs
                while queue.is_empty() {
                    queue = cvar.wait(queue).unwrap();
                }

                let mut job = queue.pop().unwrap();
                drop(queue); // release lock before executing job

                println!("Worker {} executing priority {}", id, job.priority);

                if let Some(task) = job.task.take() {
                    task();
                }
            }
        });

        Worker { _id: id }
    }
}