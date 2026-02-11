## Day 3 — Error Handling & Reliability

### Goal
Learn how to handle failures safely in Rust using `Option`, `Result`, and custom error types.

### What I Studied
- Unrecoverable errors using `panic!`
- Recoverable errors with `Result`
- Handling missing values using `Option`
- Error propagation with the `?` operator
- When to panic vs when to return errors

### What I Implemented
- Functions returning `Option` for optional values
- File reading using `Result`
- Custom application error enum
- Error conversion using `From`
- Functions using `?` for propagation
- Controlled `panic!` example

### Problems Faced
- Confusion between `Option` and `Result`
- Handling multiple error types
- Understanding how `?` propagates errors
- Deciding when to use `panic!`

### How I Solved Them
- Practiced matching on `Option` and `Result`
- Used custom error enums to unify errors
- Traced error flow manually through functions
- Studied examples from the Rust Book

### Key Insight
Good Rust programs treat errors as part of the API. Instead of hiding failures, they make them explicit and enforce handling at compile time.

### Tomorrow’s Plan
Study traits and generics to design reusable and extensible interfaces.
