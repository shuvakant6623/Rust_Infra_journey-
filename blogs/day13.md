## Day 13 — Introducing Backpressure

### Goal

Today I implemented bounded queues and producer backpressure in the thread pool.

Previously, jobs could be pushed into the queue without any limits. While this worked for simple cases, it could cause the queue to grow indefinitely if producers submitted tasks faster than workers could process them.

To address this, I introduced a queue capacity limit.

---

### Concepts Studied

- Producer–consumer synchronization
- Condition variables
- Backpressure and flow control
- Shared-state concurrency

These concepts are widely used in real infrastructure systems where stability under load is critical.

---

### Implementation

A `capacity` field was added to the shared state.

When a producer submits a job:

1. The shared state is locked.
2. If the queue is full, the producer waits on the condition variable.
3. When workers remove jobs from the queue, they notify the condition variable.
4. Producers wake up and attempt submission again.

This guarantees that the queue never grows beyond its configured capacity.

---

### Key Invariant

The system must always maintain the following condition:

queue.len() <= capacity

This invariant ensures bounded memory usage.

---

### Worker Behavior

Workers perform the following steps:

1. Acquire the lock
2. Wait if the queue is empty
3. Pop the highest priority job
4. Notify producers that queue space is available
5. Execute the job

Workers also periodically wake up using `wait_timeout` to detect idle periods.

---

### Challenges

Correctly using condition variables required careful reasoning.

Two key lessons:

- `while` loops must be used instead of `if` to handle spurious wakeups.
- workers must notify producers after removing jobs from the queue.

Failure to follow these patterns can lead to deadlocks.

---

### Key Insight

Concurrency is not just about parallel execution.

It is also about **resource control and system stability**.

Bounded queues introduce natural flow control and prevent producers from overwhelming the system.

This pattern is widely used in production infrastructure such as task schedulers and distributed systems.

---

### What’s Next

Next step: implement **panic isolation** so that worker threads survive if a task panics.

In production worker pools, jobs should never be able to crash the worker threads.