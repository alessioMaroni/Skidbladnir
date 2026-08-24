with Interfaces.C; use Interfaces.C;

package Time is
   pragma Pure (Time);

   function Sum_Integer (A : int; B : int) return int
     with Export        => True,
          Convention    => C,
          External_Name => "ada_sum_integer";

end Time;