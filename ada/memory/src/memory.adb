with Interfaces; use Interfaces;
with Interfaces.C; use Interfaces.C;
with Ada.Unchecked_Conversion;

package body Memory is

   function Remove_From_Freelist
     (Self   : Buddy_Allocator_Ptr;
      Order  : Interfaces.Unsigned_64;
      Target : Free_Node_Ptr) return Interfaces.C.int
   is
      Idx  : constant Integer := Integer (Order);
      Curr : Free_Node_Ptr;
      Prev : Free_Node_Ptr := null;
   begin
      if Self = null then
         return 0;
      end if;

      Curr := Self.Free_Lists (Idx);

      while Curr /= null loop
         if Curr = Target then
            if Prev = null then
               Self.Free_Lists (Idx) := Curr.Next;
            else
               Prev.Next := Curr.Next;
            end if;

            return 1;
         end if;

         Prev := Curr;
         Curr := Curr.Next;
      end loop;

      return 0;
   end Remove_From_Freelist;

   procedure Compute_Buddy_Address
     (Self            : Buddy_Allocator_Ptr;
      Current_Address : in out U8_Ptr;
      Base_Address    : in     Interfaces.Unsigned_64;
      Order           : in out Interfaces.Unsigned_64;
      Page_Size       : in     Interfaces.Unsigned_64)
   is
      function To_U64 is new Ada.Unchecked_Conversion (U8_Ptr, Interfaces.Unsigned_64);
      function To_U8_Ptr is new Ada.Unchecked_Conversion (Interfaces.Unsigned_64, U8_Ptr);
      function To_Node_Ptr is new Ada.Unchecked_Conversion (Interfaces.Unsigned_64, Free_Node_Ptr);

      Curr_Addr_Val : Interfaces.Unsigned_64 := To_U64 (Current_Address);
      Block_Offset  : Interfaces.Unsigned_64;
      Buddy_Offset  : Interfaces.Unsigned_64;
      Buddy_Addr    : Interfaces.Unsigned_64;
   begin
      while Order < Interfaces.Unsigned_64 (Max_Order - 1) loop
         Block_Offset := Curr_Addr_Val - Base_Address;
         Buddy_Offset := Block_Offset xor (Shift_Left (1, Natural (Order)) * Page_Size);
         Buddy_Addr   := Base_Address + Buddy_Offset;

         if Remove_From_Freelist (Self, Order, To_Node_Ptr (Buddy_Addr)) = 1 then
            Curr_Addr_Val := Interfaces.Unsigned_64'Min (Curr_Addr_Val, Buddy_Addr);
            Order         := Order + 1;
         else
            exit;
         end if;
      end loop;

      Current_Address := To_U8_Ptr (Curr_Addr_Val);
   end Compute_Buddy_Address;

end Memory;