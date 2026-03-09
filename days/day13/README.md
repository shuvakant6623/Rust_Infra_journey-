# Day 13 — Bounded Queue & Backpressure

## Objective

Introduce **bounded queue capacity** and **producer backpressure** into the thread pool.

Until Day 12, jobs could be submitted indefinitely, allowing the queue to grow without limits. This is dangerous in real systems because unbounded queues can lead to excessive memory usage and unstable performance.

Day 13 adds **flow control** to ensure that the system remains stable under load.

---

## Core Idea

The thread pool now follows a classic **producer–consumer model**.

Workers:
- Wait when the queue is empty
- Execute jobs when available

Producers:
- Wait when the queue is full
- Submit jobs when space becomes available

---

## Shared State

The shared state structure contains:

- Job queue (`BinaryHeap`)
- Shutdown flag
- Queue capacity

Invariant enforced:

queue.len() <= capacity

This guarantees that the queue size never exceeds the configured limit.

---

## Synchronization Strategy

The thread pool uses:

- `Mutex` to protect shared state
- `Condvar` for coordination between threads

Workers block when:

queue.is_empty()

Producers block when:

queue.len() >= capacity

Workers notify producers when they remove jobs from the queue.

---

## Priority Scheduling

The queue uses `BinaryHeap` to schedule jobs based on priority.

Higher priority jobs are executed before lower priority jobs.

This allows the thread pool to behave more like a **task scheduler** rather than a simple FIFO queue.

---

## Idle Worker Detection

Workers use `Condvar::wait_timeout` to periodically wake up when no jobs are available.

This allows the system to:

- detect idle workers
- avoid permanent blocking
- log inactivity for debugging or monitoring

---

## Graceful Shutdown

When the thread pool is dropped:

1. Shutdown flag is set
2. All workers are notified
3. Workers finish remaining jobs
4. Worker threads exit safely

This ensures that no threads remain running after the program exits.

---

## Why Backpressure Matters

Backpressure prevents system overload.

Without it:

- Producers can flood the queue
- Memory usage increases indefinitely
- Latency becomes unpredictable

Bounded queues are used in many real systems including:

- web servers
- task schedulers
- message brokers
- database worker pools

---

## Key Insight

Concurrency is not only about running tasks in parallel.

It is also about **controlling resource usage and maintaining system stability**.

Day 13 introduces **flow control**, which is a fundamental principle in infrastructure systems.