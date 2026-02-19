mod thread_pool;

use thread_pool::ThreadPool;
use std::thread;
use std::time::Duration;

fn main() {
    println!("--- Day 8: Thread Pool with Results ---");

    let pool = ThreadPool::new(3);

    // Submit simple tasks
    let mut receivers = vec![];

    for i in 0..5 {
        let rx = pool.execute_with_result(move || {
            thread::sleep(Duration::from_millis(500));
            i * 2
        });

        receivers.push(rx);
    }

    // Collect results
    for rx in receivers {
        let result = rx.recv().unwrap();
        println!("Received result: {}", result);
    }

    println!("--- End of Day 8 ---");
}
