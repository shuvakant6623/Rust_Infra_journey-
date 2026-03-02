## Day 12 — Adding Time Awareness to the Scheduler

### Goal

Make the scheduler time-aware by adding timeout-based wakeups.

---

### What I Studied

- Condvar::wait_timeout
- Duration
- Spurious wakeups
- Lock release and re-acquisition during wait

---

### What I Implemented

- Workers wake every 2 seconds if idle
- Logged idle state
- Maintained shutdown logic
- Ensured no unwrap usage
- Preserved priority scheduling

---

### Key Insight

Concurrency has two dimensions:

1. Events (job arrival)
2. Time (timeout wakeups)

wait_timeout allows combining both.

---

### Why While Loop Is Still Required

Even with timeout:

- Spurious wakeups can occur
- State must always be re-checked

The while loop enforces correctness.

---

### What I Now Understand

- How Condvar temporarily releases lock
- Why guard must be returned
- How time-based systems operate
- How production schedulers detect idle state

---

### Tomorrow’s Plan

Implement job cancellation or bounded queue.