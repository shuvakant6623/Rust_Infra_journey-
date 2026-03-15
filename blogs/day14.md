## Day 14 — Worker Panic Isolation

### Goal

Ensure worker threads survive task panics.

Previously, if a task panicked, the panic would unwind the stack and terminate the worker thread.

This caused the thread pool to permanently lose workers.

---

### Concept Studied

Rust panic behavior.

Rust panics unwind the stack until a boundary is reached. If no boundary exists, the thread terminates.

To prevent this, I wrapped task execution in a panic boundary using `catch_unwind`.

---

### Implementation

When executing a task:

1. The closure is wrapped inside `catch_unwind`.
2. If the closure panics, the error is captured.
3. The worker logs the panic but continues execution.

This prevents worker threads from dying.

---

### Why This Matters

Workers execute user-provided tasks which may contain bugs.

Infrastructure systems must assume tasks are unreliable and protect the scheduler.

---

### Key Insight

Workers should be treated as long-lived infrastructure components.

User tasks are transient and must never be allowed to kill the worker.