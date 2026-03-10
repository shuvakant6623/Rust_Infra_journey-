mod thread_pool;

use thread_pool::ThreadPool;
use std::thread;
use std::time::Duration;

fn main() {

    let pool = ThreadPool::new(3);

    println!("Submitting jobs...\n");

    for i in 0..15 {

        let priority = (i % 5) + 1;

        println!("Submitting job {} with priority {}", i, priority);

        pool.execute(priority, move || {

            println!(
                "Job {} (priority {}) started on thread {:?}",
                i,
                priority,
                thread::current().id()
            );

            thread::sleep(Duration::from_millis(800));

            println!("Job {} finished", i);

        });
    }

    println!("\nAll jobs submitted.");

    thread::sleep(Duration::from_secs(10));

    println!("Main exiting...");
}