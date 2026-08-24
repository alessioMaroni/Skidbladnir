use core::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;

pub const MAX_ORDER: usize = 11;
pub const PAGE_SIZE: usize = 4096;

pub struct FreeNode {
    pub next: *mut FreeNode,
}

pub struct BuddyAllocator {
    free_lists: [*mut FreeNode; MAX_ORDER],
    base_addr: u64,
}

impl BuddyAllocator {
    pub const fn new() -> Self {
        Self {
            free_lists: [null_mut(); MAX_ORDER],
            base_addr: 0,
        }
    }

    pub fn init(&mut self, boot_info: &crate::BootInfo) {
        self.base_addr = boot_info.fr.heap_start;

        let node = self.base_addr as *mut FreeNode;

        unsafe {
            (*node).next = null_mut();
        }

        self.free_lists[MAX_ORDER - 1] = node;
    }

    pub fn alloc(&mut self, order: usize) -> Option<*mut u8> {
        if order >= MAX_ORDER {
            return None;
        }

        for current_order in order..MAX_ORDER {
            if !self.free_lists[current_order].is_null() {
                let block = self.free_lists[current_order];
                unsafe {
                    self.free_lists[current_order] = (*block).next;
                }

                let mut size = (1 << current_order) * PAGE_SIZE;
                for j in (order..current_order).rev() {
                    size /= 2;
                    let buddy = (block as usize + size) as *mut FreeNode;
                    unsafe {
                        (*buddy).next = self.free_lists[j];
                        self.free_lists[j] = buddy;
                    }
                }

                return Some(block as *mut u8);
            }
        }

        None
    }

    pub unsafe fn dealloc(&mut self, ptr: *mut u8, mut order: usize) {
        let mut current_addr = ptr as usize;
        let base = self.base_addr as usize;

        while order < MAX_ORDER - 1 {
            let block_offset = current_addr - base;
            let buddy_offset = block_offset ^ ((1 << order) * PAGE_SIZE);
            let buddy_addr: usize = base + buddy_offset;

            if self.remove_from_freelist(order, buddy_addr as *mut FreeNode) {
                current_addr = core::cmp::min(current_addr, buddy_addr);
                order += 1;
            } else {
                break;
            }
        }

        let node = current_addr as *mut FreeNode;
        unsafe {
            (*node).next = self.free_lists[order];
        }
        self.free_lists[order] = node;
    }

    fn remove_from_freelist(&mut self, order: usize, target: *mut FreeNode) -> bool {
        let mut curr = &mut self.free_lists[order];
        while !curr.is_null() {
            if *curr == target {
                unsafe {
                    *curr = (**curr).next;
                }
                return true;
            }
            unsafe {
                curr = &mut (**curr).next;
            }
        }
        false
    }
}

pub struct LockedBuddyAllocator(spin::Mutex<BuddyAllocator>);

impl LockedBuddyAllocator {
    pub const fn new() -> Self {
        Self(spin::Mutex::new(BuddyAllocator::new()))
    }

    pub fn init(&self, boot_info: &crate::BootInfo) {
        self.0.lock().init(boot_info);
    }
}

unsafe impl GlobalAlloc for LockedBuddyAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let size = layout.size().max(layout.align());
        let pages = (size + PAGE_SIZE - 1) / PAGE_SIZE;
        let order = pages.next_power_of_two().trailing_zeros() as usize;

        self.0.lock().alloc(order).unwrap_or(null_mut())
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let size = layout.size().max(layout.align());
        let pages = (size + PAGE_SIZE - 1) / PAGE_SIZE;
        let order = pages.next_power_of_two().trailing_zeros() as usize;

        unsafe {
            self.0.lock().dealloc(ptr, order);
        }
    }
}

unsafe impl Send for BuddyAllocator {}
unsafe impl Send for LockedBuddyAllocator {}
unsafe impl Sync for LockedBuddyAllocator {}

#[global_allocator]
pub static ALLOCATOR: LockedBuddyAllocator = LockedBuddyAllocator::new();