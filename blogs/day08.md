## Day 8 — Thread Pool with Result-Returning Tasks

### Goal
Extend the thread pool to support result-returning tasks using a request–response pattern.

### What I Studied
- How to return values from concurrent tasks
- Using `mpsc` channels for two-way communication
- Generic functions with type parameters
- The relationship between `Send` and `'static` in threaded code
- Request–response patterns in concurrent systems

### What I Implemented
- Added `execute_with_result` method to the thread pool
- Created per-task response channels
- Used generics to support any return type
- Sent results back to the caller safely
- Collected and processed results in the main thread

### Problems Faced
- Understanding why return types must implement `Send`
- Managing ownership when moving closures into threads
- Handling result channels without blocking incorrectly
- Ensuring generic constraints were correct

### How I Solved Them
- Added `T: Send + 'static` bounds to result types
- Carefully moved closures using `move`
- Used `mpsc::channel` per job for clean response handling
- Traced data flow from submission → execution → result return

### Key Insight
Concurrency is not just about executing tasks in parallel. Real systems require structured communication between producers and workers, and result channels provide a clean and safe way to implement this pattern.

### Why This Matters
Result-returning thread pools are foundational in:
- Job schedulers
- Background task systems
- Execution engines
- Distributed task processing frameworks

This upgrade made the thread pool significantly more practical and closer to real-world infrastructure components.

### Tomorrow’s Plan
Enhance the thread pool with priority scheduling or introduce timeouts for long-running tasks.
