## Day 4 — Traits & Generic Abstractions

### Goal
Learn how to design reusable and extensible components in Rust using traits and generics.

### What I Studied
- Generic functions and structs
- Trait definitions and implementations
- Trait bounds and `where` clauses
- Static vs dynamic dispatch
- Trait objects using `dyn`

### What I Implemented
- A custom `Engine` trait
- Multiple implementations for different engines
- Generic functions with trait bounds
- Generic executor struct
- Runtime polymorphism using `Box<dyn>`
- Utility function using `Debug` trait

### Problems Faced
- Understanding trait bounds syntax
- Confusion between static and dynamic dispatch
- Lifetime and trait interactions
- Designing clean generic APIs

### How I Solved Them
- Rewrote examples using simpler trait bounds
- Compared generic functions with trait objects
- Studied compiler error messages carefully
- Refactored code to improve readability

### Key Insight
Traits and generics allow Rust programs to be both flexible and efficient. By combining compile-time safety with abstraction, Rust enables building scalable system components without sacrificing performance.

### Tomorrow’s Plan
Study concurrency using threads and channels and learn how Rust ensures thread safety.
