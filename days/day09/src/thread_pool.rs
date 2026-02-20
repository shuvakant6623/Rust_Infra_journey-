use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::sync::{Arc, Mutex};
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
    queue: Arc<Mutex<BinaryHeap<Job>>>,
}

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        assert!(size > 0);

        let queue = Arc::new(Mutex::new(BinaryHeap::new()));
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&queue)));
        }

        ThreadPool { workers, queue }
    }

    pub fn execute<F>(&self, priority: usize, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Job {
            priority,
            task: Some(Box::new(f)),
        };

        let mut queue = self.queue.lock().unwrap();
        queue.push(job);
    }
}

struct Worker {
    _id: usize,
}

impl Worker {
    fn new(id: usize, queue: Arc<Mutex<BinaryHeap<Job>>>) -> Self {
        thread::spawn(move || loop {
            let job = {
                let mut q = queue.lock().unwrap();
                q.pop()
            };

            if let Some(mut job) = job {
                println!("Worker {} executing priority {}", id, job.priority);

                if let Some(task) = job.task.take() {
                    task();
                }
            }
        });

        Worker { _id: id }
    }
}