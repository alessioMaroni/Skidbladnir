# Buddy Allocator

* [**Buddy Allocator**](./../src/mm/buddy)

Yggdrasil uses the buddy allocator as the sole `#[global_allocator]` for the kernel. There is currently no secondary allocator.

## Why It Works For Now
* **Minimum page size (4096 bytes)**: This works well as long as the kernel primarily allocates large structures.

## The Limit: Internal Fragmentation on Small Objects
If the kernel were to allocate many 32–64 byte objects (e.g., thousands of small structs per process), each allocation would occupy a full page, resulting in severe internal fragmentation.

## Usage of the Ada/SPARK Language
Integrating Ada into the memory allocator aims to improve security and guarantee deterministic behavior. This approach leverages the strong memory safety guarantees of Ada alongside the formal mathematical verifiability provided by SPARK.

## Next Steps (Planned)
* Implement a slab allocator layered on top of the existing buddy allocator for when the kernel begins allocating smaller objects.
* Implement SPARK verification for deterministic memory allocation.