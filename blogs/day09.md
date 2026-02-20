## Day 9 — Adding Priority Scheduling to the Thread Pool

### Goal
Extend the thread pool to support priority-based job execution using a BinaryHeap.

### What I Studied
- BinaryHeap from the standard library
- The Ord, PartialOrd, PartialEq, and Eq traits
- How Rust determines ordering in heaps
- Trait implementation rules for custom structs

### What I Implemented
- Created a Job struct with a priority field
- Implemented custom Ord and PartialOrd for heap ordering
- Used BinaryHeap as a max-heap scheduler
- Modified worker logic to pop highest-priority jobs first
- Integrated priority-based execution into the thread pool

### Problems Faced
- Syntax error in partial_cmp implementation
- derive(Eq) failing due to FnOnce inside Job
- Understanding why closures cannot implement Eq
- Compiler trait-bound errors

### How I Solved Them
- Fixed incorrect function signature syntax
- Removed derive(Eq) and manually implemented Eq
- Compared jobs only by priority, not by task
- Carefully read compiler error explanations

### Key Insight
Scheduling is not just about executing tasks; it is about deciding which task runs first. By implementing custom ordering, I gained deeper insight into how real systems prioritize workloads.

### Why This Matters
Priority scheduling is foundational in:
- Operating system schedulers
- Task queues
- Distributed job systems
- Infrastructure workload management

This step moved the thread pool closer to real-world scheduling systems.

### Tomorrow’s Plan
Eliminate the busy-wait loop and implement a blocking scheduler using Condvar for more efficient execution.