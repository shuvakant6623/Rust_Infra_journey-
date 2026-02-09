## Day 1 — Ownership & Memory Model

### Goal
Understand how Rust manages memory using ownership and borrowing.

### What I Studied
- Stack vs Heap
- Move vs Copy
- Ownership rules
- Borrowing

### What I Implemented
- Move and clone examples
- Ownership transfer in functions
- Immutable and mutable borrowing

### Problems Faced
- Use-after-move compiler error
- Mutable borrow conflict

### How I Solved Them
- Used references instead of moving
- Scoped mutable borrows properly

### Key Insight
Rust enforces memory safety at compile time, which changes how programs are designed.

### Tomorrow’s Plan
Deep dive into lifetimes and advanced borrowing.
