# Mutex Documentation

* [**Mutex**](../../src/sync/mutex.rs)

## Overview

A mutex (short for **mutual exclusion**) is a synchronization primitive used in concurrent programming to ensure that only one thread or CPU core can access a shared resource or execution path at a time.

In the **ToPluto Kernel Project**, this module provides a custom, lightweight, spinlock-based mutex designed for `no_std` kernel environments without relying on operating system threading primitives or external allocation crates.

* **Spin-Locking Mechanism:** Unlike standard user-space OS mutexes that sleep/suspend threads, this kernel implementation busy-waits (spins) using atomic operations and `core::hint::spin_loop()` until the lock becomes available.
* **RAII Memory Safety:** Resource access is bound to the lifetime of a `MutexGuard`. Access to the inner data is provided via dereferencing, and the lock is automatically released when the guard drops out of scope.
* **Interior Mutability:** Wraps data inside an `UnsafeCell<T>` while guaranteeing exclusive mutable access across cores via `AtomicBool` synchronization flags with `Acquire`/`Release` memory ordering.

---

## Data Structures

### `Mutex<T>`
The core synchronization primitive wrapping generic data `T`.

```rust
pub struct Mutex<T> {
    lock: AtomicBool,
    data: UnsafeCell<T>,
}

---

## Usage

<!-- TODO: Add the usage inside the kernel -->
