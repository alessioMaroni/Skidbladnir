with Interfaces;
with Interfaces.C;

package Memory is

   Max_Order : constant := 11;
   Page_Size : constant := 4096;

   type U8_Ptr is access all Interfaces.Unsigned_8;
   pragma Convention (C, U8_Ptr);
   pragma No_Strict_Aliasing (U8_Ptr);

   type Free_Node;
   type Free_Node_Ptr is access all Free_Node;
   pragma Convention (C, Free_Node_Ptr);

   type Free_Node is record
      Next : Free_Node_Ptr;
   end record;
   pragma Convention (C, Free_Node);

   type Free_Lists_Array is array (0 .. Max_Order - 1) of Free_Node_Ptr;
   pragma Convention (C, Free_Lists_Array);

   type Buddy_Allocator is record
      Free_Lists : Free_Lists_Array;
      Base_Addr  : Interfaces.Unsigned_64;
   end record;
   pragma Convention (C, Buddy_Allocator);

   type Buddy_Allocator_Ptr is access all Buddy_Allocator;
   pragma Convention (C, Buddy_Allocator_Ptr);

   function Remove_From_Freelist
     (Self   : Buddy_Allocator_Ptr;
      Order  : Interfaces.Unsigned_64;
      Target : Free_Node_Ptr) return Interfaces.C.int;
   pragma Export (Convention    => C,
                  Entity        => Remove_From_Freelist,
                  External_Name => "ada_remove_from_freelist");

   procedure Compute_Buddy_Address
     (Self            : Buddy_Allocator_Ptr;
      Current_Address : in out U8_Ptr;
      Base_Address    : in     Interfaces.Unsigned_64;
      Order           : in out Interfaces.Unsigned_64;
      Page_Size       : in     Interfaces.Unsigned_64);
   pragma Export (Convention    => C,
                  Entity        => Compute_Buddy_Address,
                  External_Name => "ada_compute_buddy_address");

end Memory;