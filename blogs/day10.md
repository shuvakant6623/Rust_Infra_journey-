## Day 10 — Implementing a Blocking Priority Scheduler

### Goal
Replace the busy-wait loop with a proper blocking scheduler using Condvar.

### What I Studied
- Condition variables in Rust
- Producer–consumer concurrency pattern
- Efficient synchronization with Mutex and Condvar

### What I Implemented
- Combined BinaryHeap with Condvar
- Workers wait when queue is empty
- Producer notifies workers when new job arrives
- Released lock before executing tasks

### Problems Faced
- Understanding how Condvar.wait works
- Avoiding deadlocks
- Knowing when to drop the mutex lock

### How I Solved Them
- Carefully followed the lock → while empty → wait pattern
- Ensured locks are dropped before task execution
- Reviewed shared-state concurrency concepts

### Key Insight
Efficient concurrency is not just about parallel execution, but about sleeping intelligently when there is no work. Blocking synchronization eliminates wasted CPU cycles.

### Tomorrow’s Plan
Add graceful shutdown and termination signaling to the blocking scheduler.