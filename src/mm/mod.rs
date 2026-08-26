

pub mod buddy {
    pub mod loked_buddy_impl;
    pub mod global_impl;
    pub mod definitions;
    pub mod buddy_impl;
}

pub mod bump {
    
}

use crate::mm::buddy::definitions::LockedBuddyAllocator;

/// Global kernel allocator instance for `alloc` data structures (`Box`, `Vec`, etc.).
#[global_allocator]
pub static ALLOCATOR: LockedBuddyAllocator = LockedBuddyAllocator::new();