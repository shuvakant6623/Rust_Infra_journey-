mod thread_pool;

use thread_pool::ThreadPool;
use std::thread;
use std::time::Duration;

fn main() {
    println!("--- Day 7: Production-Style Thread Pool ---");

    let pool = ThreadPool::new(3);

    for i in 0..6 {
        pool.execute(move || {
            println!("Job {} running...", i);
            thread::sleep(Duration::from_millis(500));
            println!("Job {} done.", i);
        });
    }

    thread::sleep(Duration::from_secs(2));

    println!("Main: Active jobs remaining: {}", pool.active_job_count());

    println!("--- End of Day 7 ---");
}
