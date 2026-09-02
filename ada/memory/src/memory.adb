-- Copyright (c) 2026 ToPluto Kernel Project
--
-- Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
-- https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
-- <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
-- option. This file may not be copied, modified, or distributed
-- except according to those terms.

-- Implementation of the Buddy Allocator package body (`memory.adb`).
--
-- Provides the execution logic for managing free block lists, removing nodes,
-- and computing buddy addresses during recursive memory coalescing.

with System;
with Interfaces; use Interfaces;
with Interfaces.C; use Interfaces.C;
with Ada.Unchecked_Conversion;
with System.Storage_Elements;
with Ada.Unchecked_Conversion;

package body Memory is

    -- Remove_From_Freelist --
	--
    -- Removes a target node from the free list of a given order.
    --
    -- Parameters:
    --   Self   : Pointer to the allocator instance.
    --   Order  : Memory order level (list index) from which to remove the node.
    --   Target : Pointer to the node that needs to be detached.
    --
    -- Returns:
    --   1 if the target node was found and removed successfully.
    --   0 if the allocator instance is null or the node was not found.
    function Remove_From_Freelist
        (Self   : Buddy_Allocator_Ptr;
         Order  : Interfaces.Unsigned_64;
         Target : Free_Node_Ptr) return Interfaces.C.int
    is
        Idx  : constant Integer := Integer (Order);
        Curr : Free_Node_Ptr;
        Prev : Free_Node_Ptr := null;
    begin
        -- Validate allocator pointer
        if Self = null then
            return 0;
        end if;

        Curr := Self.Free_Lists (Idx);

        -- Traverse the singly-linked list to find the Target node
        while Curr /= null loop
            if Curr = Target then
                -- Detach node from list
                if Prev = null then
                    -- Target is the head of the list
                    Self.Free_Lists (Idx) := Curr.Next;
                else
                    -- Target is in the middle or end of the list
                    Prev.Next := Curr.Next;
                end if;

                return 1; -- Successfully removed
            end if;

            Prev := Curr;
            Curr := Curr.Next;
        end loop;

        return 0; -- Node not found in the freelist
    end Remove_From_Freelist;

    -- Search_Free_Block
    --
    -- Search_Free_Block: Searches for a free memory block matching the requested order.
    --
    -- Splits larger blocks (buddy splitting) and populates intermediate free lists.
    --
    -- Parameters: Self (allocator pointer), Order (allocation scale).
    --
    -- Return: Memory block address, or Null_Address if out of memory or invalid.
    --
    -- FFI: Exported with C ABI, mapping seamlessly to Rust's Option<*mut u8>.
    function Search_Free_Block
      (Self  : Buddy_Allocator_Ptr;
       Order : Interfaces.Unsigned_64) return System.Address 
    is
       -- Unchecked conversions for safe pointer arithmetic
       function To_U64 is new Ada.Unchecked_Conversion (Free_Node_Ptr, Interfaces.Unsigned_64);
       function To_Node_Ptr is new Ada.Unchecked_Conversion (Interfaces.Unsigned_64, Free_Node_Ptr);
       function To_Address is new Ada.Unchecked_Conversion (Free_Node_Ptr, System.Address);

       Block         : Free_Node_Ptr := null;
       Buddy         : Free_Node_Ptr := null;
       Size          : Interfaces.Unsigned_64;
       Current_Order : Integer;
    begin
       if Order >= Interfaces.Unsigned_64 (Max_Order) then
          return System.Null_Address;
       end if;

       -- Max_Order is 11, so the array goes up to 10
       for I in Integer (Order) .. Max_Order - 1 loop
          Current_Order := I;

          if Self.Free_Lists (Current_Order) /= null then
             -- Fetch the first available block from the current order
             Block := Self.Free_Lists (Current_Order);
             Self.Free_Lists (Current_Order) := Block.Next;

             -- Recursively split down to the requested order
             Size := Shift_Left (Interfaces.Unsigned_64 (1), Current_Order) * Interfaces.Unsigned_64 (Page_Size);

             for J in reverse Integer (Order) .. (Current_Order - 1) loop
                Size  := Size / 2;
                Buddy := To_Node_Ptr (To_U64 (Block) + Size);
                Buddy.Next := Self.Free_Lists (J);
                Self.Free_Lists (J) := Buddy;
             end loop;

             return To_Address (Block);
          end if;
       end loop;

       return System.Null_Address;
    end Search_Free_Block;

    -- Compute_Buddy_Address
	--
    -- Calculates buddy memory addresses and performs iterative merging (coalescing)
    -- up to the maximum allocator order.
    --
    -- Parameters:
    --   Self            : Pointer to the allocator instance.
    --   Current_Address : [In/Out] Address of the block being freed. Updated to
    --                     the base address of the combined larger block.
    --   Base_Address    : Base starting physical/virtual address of the heap.
    --   Order           : [In/Out] Order level of the block. Incremented upon each
    --                     successful merge.
    --   Page_Size       : Base architecture page size in bytes.
    procedure Compute_Buddy_Address
        (Self            : Buddy_Allocator_Ptr;
         Current_Address : in out U8_Ptr;
         Base_Address    : in     Interfaces.Unsigned_64;
         Order           : in out Interfaces.Unsigned_64;
         Page_Size       : in     Interfaces.Unsigned_64)
    is
        -- Unchecked conversions between raw integer addresses and access pointers
        function To_U64 is new Ada.Unchecked_Conversion
		 (U8_Ptr, Interfaces.Unsigned_64);
        function To_U8_Ptr is new Ada.Unchecked_Conversion
		 (Interfaces.Unsigned_64, U8_Ptr);
        function To_Node_Ptr is new Ada.Unchecked_Conversion
		 (Interfaces.Unsigned_64, Free_Node_Ptr);

        Curr_Addr_Val : Interfaces.Unsigned_64 := To_U64 (Current_Address);
        Block_Offset  : Interfaces.Unsigned_64;
        Buddy_Offset  : Interfaces.Unsigned_64;
        Buddy_Addr    : Interfaces.Unsigned_64;
    begin
        -- Iteratively attempt to merge with adjacent buddies up to Max_Order - 1
        while Order < Interfaces.Unsigned_64 (Max_Order - 1) loop
            -- Calculate offset relative to the heap base address
            Block_Offset := Curr_Addr_Val - Base_Address;

            -- Calculate buddy offset using XOR: (1 << Order) * Page_Size gives block size
            Buddy_Offset := Block_Offset xor (Shift_Left (1, Natural (Order)) * Page_Size);
            Buddy_Addr   := Base_Address + Buddy_Offset;

            -- Check if buddy is free and available in the corresponding freelist
            if Remove_From_Freelist (Self, Order, To_Node_Ptr (Buddy_Addr)) = 1 then
                -- Merge successful: set address to the lowest base address of the pair
                Curr_Addr_Val := Interfaces.Unsigned_64'Min (Curr_Addr_Val, Buddy_Addr);
                -- Promote block to the next higher order
                Order         := Order + 1;
            else
                -- Buddy is allocated or unavailable; stop coalescing
                exit;
            end if;
        end loop;

        -- Update output address to point to the base of the coalesced block
        Current_Address := To_U8_Ptr (Curr_Addr_Val);
    end Compute_Buddy_Address;

end Memory;
