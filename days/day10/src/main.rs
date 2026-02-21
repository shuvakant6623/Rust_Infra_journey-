mod thread_pool;

use thread_pool::ThreadPool;
use std::thread;
use std::time::Duration;

fn main() {
    println!("--- Day 10: Blocking Priority Scheduler ---");

    let pool = ThreadPool::new(3);

    pool.execute(1, || {
        println!("Low priority job");
        thread::sleep(Duration::from_millis(500));
    });

    pool.execute(5, || {
        println!("High priority job");
        thread::sleep(Duration::from_millis(500));
    });

    pool.execute(3, || {
        println!("Medium priority job");
        thread::sleep(Duration::from_millis(500));
    });

    thread::sleep(Duration::from_secs(3));
}