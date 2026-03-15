# Day 14 — Panic Isolation

## Objective

Ensure that worker threads survive if a task panics.

In earlier versions, if a task panicked the entire worker thread would crash. This would permanently reduce the thread pool's capacity.

Day 14 introduces panic isolation.

---

## Problem

User tasks are untrusted.

They may panic due to bugs or invalid assumptions. If panic propagates to the worker thread, the thread terminates.

Example failure chain:

task panic → worker thread dies → fewer workers → degraded system performance

---

## Solution

Wrap job execution inside a panic boundary using:

std::panic::catch_unwind

If a panic occurs:

1. The panic is caught
2. An error is logged
3. The worker loop continues running

---

## Result

Worker threads become resilient to task failures.

This pattern is common in:

- job schedulers
- web servers
- background workers
- distributed task executors

---

## Key Insight

Worker threads must treat tasks as **untrusted code**.

Infrastructure systems must isolate task failures so that the scheduler remains stable.