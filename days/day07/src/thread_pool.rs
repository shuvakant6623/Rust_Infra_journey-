use std::sync::{mpsc, Arc, Mutex};
use std::thread;

type Job = Box<dyn FnOnce() + Send + 'static>;

fn log(message: &str) {
    println!("[ThreadPool] {}", message);
}

enum Message {
    NewJob(Job),
    Terminate,
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
    active_jobs: Arc<Mutex<usize>>,
}

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let active_jobs = Arc::new(Mutex::new(0));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(
                id,
                Arc::clone(&receiver),
                Arc::clone(&active_jobs),
            ));
        }

        log(&format!("Initialized with {} workers", size));

        ThreadPool {
            workers,
            sender,
            active_jobs,
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        if let Err(e) = self.sender.send(Message::NewJob(job)) {
            log(&format!("Failed to send job: {}", e));
        }
    }

    pub fn active_job_count(&self) -> usize {
        *self.active_jobs.lock().unwrap()
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        log("Sending terminate signal to workers.");

        for _ in &self.workers {
            if let Err(e) = self.sender.send(Message::Terminate) {
                log(&format!("Failed to send terminate signal: {}", e));
            }
        }

        for worker in &mut self.workers {
            log(&format!("Shutting down worker {}", worker.id));

            if let Some(thread) = worker.thread.take() {
                if let Err(e) = thread.join() {
                    log(&format!("Worker {} join error: {:?}", worker.id, e));
                }
            }
        }

        log("Shutdown complete.");
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(
        id: usize,
        receiver: Arc<Mutex<mpsc::Receiver<Message>>>,
        active_jobs: Arc<Mutex<usize>>,
    ) -> Self {
        let thread = thread::spawn(move || loop {
            let message = {
                let lock = receiver.lock();
                if lock.is_err() {
                    log(&format!("Worker {} failed to lock receiver", id));
                    break;
                }

                lock.unwrap().recv()
            };

            match message {
                Ok(Message::NewJob(job)) => {
                    {
                        let mut count = active_jobs.lock().unwrap();
                        *count += 1;
                        log(&format!("Worker {} started job. Active jobs: {}", id, *count));
                    }

                    job();

                    {
                        let mut count = active_jobs.lock().unwrap();
                        *count -= 1;
                        log(&format!("Worker {} finished job. Active jobs: {}", id, *count));
                    }
                }

                Ok(Message::Terminate) => {
                    log(&format!("Worker {} received terminate signal.", id));
                    break;
                }

                Err(_) => {
                    log(&format!("Worker {} channel disconnected.", id));
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
