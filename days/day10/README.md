# Day 10 — Blocking Priority Scheduler with Condvar

Focus:
- Condition variables
- Blocking worker threads
- Efficient producer-consumer pattern
- Removing busy-wait loops

Code demonstrates:
- BinaryHeap priority queue
- Mutex + Condvar coordination
- Proper lock release before job execution
- Efficient thread wake-up mechanism

Goal:
Upgrade the scheduler to an efficient blocking design suitable for real infrastructure systems.