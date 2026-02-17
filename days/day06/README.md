# Day 6 — Thread Pool Implementation

Focus:
- Fixed-size worker threads
- Job scheduling via channels
- Shared receiver using Arc and Mutex
- Graceful shutdown using Drop

Code demonstrates:
- Worker abstraction
- Task execution queue
- Thread reuse
- Clean resource cleanup

Goal:
Build a reusable infrastructure component for concurrent task execution.
