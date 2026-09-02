-- Copyright (c) 2026 ToPluto Kernel Project
--
-- Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
-- https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
-- <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
-- option. This file may not be copied, modified, or distributed
-- except according to those terms.

-- Ada definition of the Buddy Allocator.
-- Address calculation during deallocation rewritten in Ada.

-- TODO: Implement SPARK for mathematical verification

with System;
with Interfaces;
with Interfaces.C;

package Memory is

	-- Maximum number of orders (levels) managed by the allocator (0 to MAX_ORDER - 1).
	-- With MAX_ORDER = 11, the maximum order is 10, corresponding to 2^10 = 1024 pages (4 MiB). 
	Max_Order : constant := 11;

	-- Standard memory page size on the x86_64 architecture (4096 bytes).
	Page_Size : constant := 4096;

	-- C-compatible pointer to an 8-bit unsigned integer (equivalent to 'uint8_t*').
	-- Enforces C ABI alignment and disables strict aliasing for safe raw memory access.
	type U8_Ptr is access all Interfaces.Unsigned_8;
  	pragma Convention (C, U8_Ptr);
   	pragma No_Strict_Aliasing (U8_Ptr);

	-- An intrusive node stored directly inside free memory blocks.
	--
	-- Forms a singly linked list for each order inside 'Free_Lists_Array'.
  	type Free_Node;
  	type Free_Node_Ptr is access all Free_Node;
   	pragma Convention (C, Free_Node_Ptr);
	type Free_Node is record
      		Next : Free_Node_Ptr;
  	end record;
   	pragma Convention (C, Free_Node);

	-- Array of free memory block lists for each order.
   	type Free_Lists_Array is array (0 .. Max_Order - 1) of Free_Node_Ptr;
   	pragma Convention (C, Free_Lists_Array);

	-- Main Buddy Allocator structure (non-thread-safe).
   	type Buddy_Allocator is record
		-- Array of linked lists of free blocks for each order.
      		Free_Lists : Free_Lists_Array;
		-- Starting memory address (physical or virtual) of the heap.
      		Base_Addr  : Interfaces.Unsigned_64;
   	end record;
   	pragma Convention (C, Buddy_Allocator);

	-- Pointer to the Buddy Allocator structure.
	type Buddy_Allocator_Ptr is access all Buddy_Allocator;
  	pragma Convention (C, Buddy_Allocator_Ptr);

	-- Removes a specific target node from the free list of the specified order.
    --
    -- Used during deallocation to extract the buddy prior to merging.
   	function Remove_From_Freelist
	    (Self   : Buddy_Allocator_Ptr;
         Order  : Interfaces.Unsigned_64;
      	 Target : Free_Node_Ptr) return Interfaces.C.int;

    -- Serach for a free Block in the Buddy Allocator 'alloc' function.
    --
    -- Designed for use in the Rust memory manager (mm/buddy) for enhanced safty
    -- and mathematical verification.
    --
    -- Exported with C-compatible external name 'ada_search_free_block'
    function Search_Free_Block
     (Self  : Buddy_Allocator_Ptr;
      Order : Interfaces.Unsigned_64) return System.Address;
    pragma Export (
       Convention    => C,
       Entity        => Search_Free_Block,
       External_Name => "ada_search_free_block"
    );

	-- Calculates the buddy address and attempts merging with higher orders.
    --
	-- Designed for use in the Rust memory manager (mm/buddy allocator) for enhanced safety
	-- and mathematical verification.
    --
	-- Exported with the C-compatible external name 'ada_compute_buddy_address'.
   	procedure Compute_Buddy_Address
     	(Self           : Buddy_Allocator_Ptr;
      	 Current_Address : in out U8_Ptr;
      	 Base_Address    : in     Interfaces.Unsigned_64;
      	 Order           : in out Interfaces.Unsigned_64;
      	 Page_Size       : in     Interfaces.Unsigned_64);
   	pragma Export (
        Convention    => C,
        Entity        => Compute_Buddy_Address,
        External_Name => "ada_compute_buddy_address"
    );

end Memory;
