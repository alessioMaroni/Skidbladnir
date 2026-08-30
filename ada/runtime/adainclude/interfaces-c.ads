with Interfaces;

package Interfaces.C is
   pragma Pure;

   type int is range -2 ** 31 .. 2 ** 31 - 1;
   for int'Size use 32;

   type unsigned is mod 2 ** 32;
   for unsigned'Size use 32;

   type size_t is mod 2 ** 64;
   for size_t'Size use 64;
end Interfaces.C;