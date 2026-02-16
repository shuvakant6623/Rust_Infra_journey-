## Day 5 — Concurrency with Threads & Channels

### Goal
Understand how Rust enables safe concurrency using threads, message passing, and shared state synchronization.

### What I Studied
- Spawning threads with `std::thread`
- Ownership transfer using the `move` keyword
- Message passing with `mpsc` channels
- Shared state concurrency using `Arc` and `Mutex`
- How Rust prevents data races at compile time

### What I Implemented
- A basic thread executing parallel tasks
- Moving owned data safely into a thread
- A producer-consumer example using channels
- A shared counter updated across multiple threads
- Synchronization using `Arc<Mutex<T>>`

### Problems Faced
- Understanding why ownership must be moved into threads
- Confusion about when to use `Arc` vs `Mutex`
- Compiler errors when trying to share mutable data
- Interpreting thread-related error messages

### How I Solved Them
- Used `move` closures to transfer ownership
- Wrapped shared data in `Arc` for multiple ownership
- Used `Mutex` to ensure safe mutation
- Carefully read compiler errors to understand borrowing rules across threads

### Key Insight
Rust enforces thread safety at compile time. Instead of discovering race conditions at runtime, the compiler prevents unsafe shared access before the program even runs.

### Tomorrow’s Plan
Build a simple thread pool to understand how real infrastructure systems manage concurrent tasks efficiently.
