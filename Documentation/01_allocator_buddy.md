# Buddy Allocator

* [**Buddy Allocator**](./../src/mm/buddy)

Skidbladnir uses the buddy allocator as the sole `#[global_allocator]` for the kernel. There is currently no overlying layer for small allocations.

## Why It Works For Now
* **Minimum page size (4096 bytes)**: This works well as long as the kernel primarily allocates large structures (such as `Vec` and I/O buffers).

## The Limit: Internal Fragmentation on Small Objects
If the kernel were to allocate many 32–64 byte objects (e.g., thousands of small structs per process), each would still consume an entire page (4096 bytes)—wasting over 98% of the space per object. This is why real-world kernels (like Linux) place a **slab allocator** on top of the buddy allocator: the buddy manages whole pages, whereas the slab allocator subdivides a page into many small objects of uniform size or type.

## Next Steps (Planned)
* Implement a slab allocator layered on top of the existing buddy allocator for when the kernel begins allocating numerous small structures (e.g., Process Control Blocks).
