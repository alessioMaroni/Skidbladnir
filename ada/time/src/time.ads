package Time is

    procedure Sum_Integer
        with Export         => True,
            Convention      => C,
            External_Name   => "ada_sum_integer";


end Time;
