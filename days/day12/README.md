# Day 12 — Timeout-Based Idle Detection

## Focus

- Condvar::wait_timeout
- Time-based wakeups
- Idle detection logging
- No unwrap usage
- Shutdown compatibility with timeouts

---

## What Changed

Workers now wake every 2 seconds if no job is available.

This prevents indefinite sleeping and introduces time-awareness.

---

## Why This Matters

Real systems need:

- Idle detection
- Periodic maintenance
- Heartbeat mechanisms
- Timeout-based behavior

Concurrency is not only event-driven.
It is also time-driven.

---

## Key Concepts

- wait_timeout returns (MutexGuard, WaitTimeoutResult)
- Always re-check condition in a while loop
- Lock is released during wait
- Lock is re-acquired before returning

---

## What This Builds Toward

- Job timeouts
- Backoff strategies
- Periodic metrics
- Cancellation tokens