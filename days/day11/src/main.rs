mod thread_pool;

use thread_pool::ThreadPool;
use std::thread;
use std::time::Duration;

fn main() {
    {
        let pool = ThreadPool::new(3);

        pool.execute(1, || {
            println!("Low priority task");
            thread::sleep(Duration::from_millis(500));
        });

        pool.execute(5, || {
            println!("High priority task");
            thread::sleep(Duration::from_millis(500));
        });

        thread::sleep(Duration::from_secs(2));
    }

    println!("Main exited cleanly.");
}