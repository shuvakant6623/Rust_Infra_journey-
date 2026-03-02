mod thread_pool;

use thread_pool::ThreadPool;
use std::thread;
use std::time::Duration;

fn main() {
    {
        let pool = ThreadPool::new(2);

        pool.execute(5, || {
            println!("High priority job running");
            thread::sleep(Duration::from_secs(1));
        });

        thread::sleep(Duration::from_secs(5));
    }

    println!("Main exited.");
}