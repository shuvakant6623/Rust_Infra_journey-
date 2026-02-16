use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};
use std::sync::mpsc;

// 1.Basic Thread

fn basic_thread() {
    let handle = thread::spawn(|| {
        for i in 1..5 {
            println!("Thread: {}", i);
            thread::sleep(Duration::from_millis(100));
        }
    });

    handle.join().unwrap();
}

// 2. Move Ownership into Thread

fn move_example() {
    let data = String::from("Rust Infra");

    let handle = thread::spawn(move || {
        println!("Moved data: {}", data);
    });

    handle.join().unwrap();
}

// 3. Message Passing (Channels)

fn channel_example() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move||{
        let msg = String::from("Hello from worker");
        tx.send(msg).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Recieved: {}", received);
}

// 4. Shared State (Arc + Mutex)

fn shared_state_example() {
    let counter = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    for _ in 0..5 {
        let counter = Arc::clone(&counter);

        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final counter: {}", *counter.lock().unwrap());
}

// Main

fn main() {
    println!("---Day 5: Concurrency---");

    basic_thread();
    move_example();
    channel_example();
    shared_state_example();

    println!("---End of day 5---");
}