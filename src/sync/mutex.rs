// Copyright (c) 2026 ToPluto Kernel Project
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! ```rust
//! use crate::sync::mutex;
//! ```
//! Custom Spinlock-based Mutex Implementation.
//!
//! This module provides a lightweight, dependency-free spinlock mutex ([`Mutex`])
//! designed for `no_std` kernel environments. Access to the underlying data
//! is controlled safely across threads/cores via an RAII guard ([`MutexGuard`]).

use core::cell::UnsafeCell;
use core::ops::{Deref, DerefMut};
use core::sync::atomic::{AtomicBool, Ordering};

/// A mutual exclusion primitive based on spin-waiting.
///
/// This struct wraps a generic value `T` inside an [`UnsafeCell`] and protects
/// access to it using an [`AtomicBool`] flag.
///
/// # Examples
///
/// ```rust
/// use crate::sync::Mutex;

/// static COUNTER: Mutex<u32> = Mutex::new(0);
///
/// fn increment() {
///     let mut guard = COUNTER.lock();
///     *guard += 1;
/// } // Lock is automatically released here when `guard` is dropped.
/// ```
pub struct Mutex<T> {
    /// Lock status flag (`true` when locked, `false` when unlocked).
    lock: AtomicBool,

    /// UnsafeCell opts data out of the compiler's strict aliasing rules
    /// to allow interior mutability.
    data: UnsafeCell<T>,
}

// Safety: `Mutex<T>` can be shared across threads/cores (`Sync`) as long as
// the underlying type `T` can be sent across thread boundaries (`Send`).
unsafe impl<T: Send> Sync for Mutex<T> {}
unsafe impl<T: Send> Send for Mutex<T> {}

/// An RAII guard returned by [`Mutex::lock`] or [`Mutex::try_lock`].
///
/// Provides mutable dereference access to the inner data while held,
/// and automatically unlocks the parent [`Mutex`] when dropped.
pub struct MutexGuard<'a, T: 'a> {
    mutex: &'a Mutex<T>,
}

impl<T> Mutex<T> {
    /// Creates a new `Mutex` protecting the given data.
    ///
    /// Marked as `const` to allow static variable initialization at compile time.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use crate::sync::Mutex;
    ///
    /// static DATA: Mutex<u32> = Mutex::new(100);
    /// let local_mutex = Mutex::new(42);
    /// ```
    pub const fn new(data: T) -> Self {
        Self {
            lock: AtomicBool::new(false),
            data: UnsafeCell::new(data),
        }
    }

    /// Acquires the lock, spinning in a tight loop until it becomes available.
    ///
    /// Returns a [`MutexGuard`] that grants exclusive mutable access to the protected data.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let mutex = Mutex::new(10);
    /// {
    ///     let mut guard = mutex.lock();
    ///     *guard = 20;
    /// }
    /// assert_eq!(*mutex.lock(), 20);
    /// ```
    pub fn lock(&self) -> MutexGuard<'_, T> {
        while self
            .lock
            .compare_exchange_weak(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            // Signals the CPU pipeline to optimize resources during spin-wait loops.
            core::hint::spin_loop();
        }

        MutexGuard { mutex: self }
    }

    /// Attempts to acquire the lock without spinning.
    ///
    /// Returns `Some(MutexGuard)` if the lock was acquired, or `None` if it was already held.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let mutex = Mutex::new(42);
    ///
    /// if let Some(mut guard) = mutex.try_lock() {
    ///     *guard += 1;
    /// } else {
    ///     // Lock was unavailable, continue execution without blocking
    /// }
    /// ```
    pub fn try_lock(&self) -> Option<MutexGuard<'_, T>> {
        if self
            .lock
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_ok()
        {
            Some(MutexGuard { mutex: self })
        } else {
            None
        }
    }

    /// Forcibly unlocks the mutex.
    ///
    /// # Safety
    ///
    /// Calling this function while another thread is actively operating on the protected
    /// data creates data races. This function is intended solely for emergency scenarios
    /// like Kernel Panics.
    pub unsafe fn force_unlock(&self) {
        self.lock.store(false, Ordering::Release);
    }
}

impl<T> Deref for MutexGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.mutex.data.get() }
    }
}

impl<T> DerefMut for MutexGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.mutex.data.get() }
    }
}

impl<T> Drop for MutexGuard<'_, T> {
    fn drop(&mut self) {
        self.mutex.lock.store(false, Ordering::Release);
    }
}