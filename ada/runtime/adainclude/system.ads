package System is
   pragma Pure;

   type Name is (System_Name_GNAT);
   System_Name : constant Name := System_Name_GNAT;

   Storage_Unit : constant := 8;
   Word_Size    : constant := 64;
   Address_Size : constant := 64;

   Min_Int      : constant := -9_223_372_036_854_775_808;
   Max_Int      : constant :=  9_223_372_036_854_775_807;

   type Address is mod 2 ** 64;
   Null_Address : constant Address := 0;

   subtype Priority is Integer range 0 .. 127;

   Max_Significand_Digits : constant := 18;
   Min_Exponent           : constant := -307;
   Max_Exponent           : constant :=  308;

   Frontend_Layout_Bug : constant Boolean := False;
   Suppress_Standard_Library_On_Target : constant Boolean := True;

private
   type Bit_Order is (High_Order_First, Low_Order_First);
   Default_Bit_Order : constant Bit_Order := Low_Order_First;
   ZCX_By_Default : constant Boolean := False;
   GCC_ZCX_Support : constant Boolean := False;
end System;