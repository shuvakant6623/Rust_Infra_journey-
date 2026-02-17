## Day 6 — Building a Thread Pool

### Goal
Understand how thread pools work internally and implement a reusable fixed-size thread pool in Rust.

### What I Studied
- The thread pool pattern
- Worker thread lifecycle
- Message passing for job scheduling
- Shared state with `Arc<Mutex<T>>`
- Graceful shutdown using `Drop`
- How production systems reuse threads instead of spawning repeatedly

### What I Implemented
- A `ThreadPool` struct managing worker threads
- A `Worker` abstraction with thread handles
- A job queue using `mpsc::channel`
- Task submission via an `execute` method
- Shared receiver wrapped in `Arc<Mutex<_>>`
- Graceful shutdown logic using `Drop`
- Multiple concurrent job executions

### Problems Faced
- Understanding why the receiver must be wrapped in both `Arc` and `Mutex`
- Handling thread shutdown correctly
- Preventing workers from blocking forever
- Interpreting compiler errors related to `Send` and `'static` bounds

### How I Solved Them
- Revisited ownership and concurrency rules
- Carefully traced which thread owns what data
- Used `Option<JoinHandle>` to safely join threads
- Studied how `FnOnce + Send + 'static` ensures safe task execution

### Key Insight
A thread pool is essentially a task scheduler built on top of message passing and controlled thread reuse. Rust’s type system ensures that tasks sent across threads are safe to execute without data races.

### Why This Matters
Thread pools are fundamental in:
- Web servers
- Data processing systems
- Execution engines
- Background job schedulers

Building one manually provided a much deeper understanding of concurrency than just using a library.

### Tomorrow’s Plan
Refactor the thread pool to improve design, add structured logging, and explore production-style improvements.
