## Day 2 — Lifetimes & Advanced Borrowing

### Goal
Understand how Rust uses lifetimes to ensure references remain valid and memory-safe.

### What I Studied
- Lifetime concepts and scope
- Lifetime annotation syntax (`'a`)
- Lifetime elision rules
- References in structs
- Advanced borrowing patterns

### What I Implemented
- Functions using explicit lifetime parameters
- `longest` function with lifetimes
- Structs holding borrowed data
- Methods returning references
- Nested scope borrowing examples

### Problems Faced
- Returning references to local variables
- Lifetime mismatch errors
- References outliving their owners
- Conflicts between mutable and immutable borrows

### How I Solved Them
- Returned owned values when needed
- Added explicit lifetime annotations
- Traced variable scopes manually
- Simplified borrowing patterns

### Key Insight
Lifetimes describe relationships between scopes, not time, and help Rust prevent dangling references at compile time.

### Tomorrow’s Plan
Study error handling using `Result` and `Option` and design reliable APIs.
