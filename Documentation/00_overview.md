# The Skidbladnir Kernel Project

> *Named after the legendary Norse ship that could adapt to any voyage, navigate any weather, and fold up when not in use.*

## What is Skidbladnir?

**Skidbladnir** is an ultra-resilient, bare-metal kernel written in **Rust** with **Ada/SPARK** integration. It is engineered for total operational adaptability across any deployment scenario:

* **Defense & Tactical:** High-reliability execution under degraded or hostile conditions.
* **Extreme Environments:** Embedded operations in maritime, aerial, and space systems.
* **Disaster Response & Edge Computing:** Mission-critical reliability during catastrophic power or system failures.
* **Consumer & Desktop:** Lightweight, predictable performance for everyday hardware at home.

---

## Project's Goals

* **Off-Grid Encrypted Communication:** Enable secure, end-to-end encrypted messaging over peer-to-peer or mesh networks without relying on internet infrastructure or central servers.
* **Resilient Remote Device Control:** Interface directly with onboard and external radio transceivers (e.g., LoRa, Sub-1GHz, RF) to command remote nodes under degraded or denied signal conditions.
* **Kernel-Level Cryptographic Pipeline:** Embed lightweight, provably secure encryption primitives natively within the kernel to prevent message interception or tampering.
* **Autonomous Ad-Hoc Networking:** Support self-healing mesh topology for distributed systems operating across maritime, airborne, or field environments.

---

## Languages We Use

### Rust
**Rust** serves as the core system implementation language for drivers, memory management, and platform initialization.

* **Compile-Time Guarantees:** Eliminates data races, use-after-free errors, and spatial memory leaks at compile time without requiring a garbage collector.
* **Zero-Cost Abstractions:** Offers high-level expressive syntax while maintaining bare-metal execution speed.
* **Robust FFI:** Interoperates smoothly with Ada and C-compatible calling conventions.

### Ada / SPARK
**Ada and SPARK** handle provably correct arithmetic, timing, and safety-critical subsystems.

* **Formal Verification:** SPARK allows mathematical proof of program properties, guaranteeing the total absence of runtime exceptions (e.g., array index out-of-bounds, arithmetic overflows).
* **Bare-Metal Execution:** Utilizes `pragma No_Run_Time` for deterministic, zero-overhead execution on bare metal.
* **Deterministic Reliability:** Preferred choice for aerospace and defense systems where failure is not an option.

---

## Target Architecture Support

| Architecture | Status | Target Triple ||
| :--- | :--- | :--- | :--- |
| **x86_64** | **Active Development** | `x86_64-unknown-none` |
| **AArch64 (ARM64)** | **Planned** | `aarch64-unknown-none` |
