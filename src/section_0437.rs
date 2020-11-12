//! @ @<Declare procedures that scan restricted classes of integers@>=
//! procedure scan_twenty_seven_bit_int;
//! begin scan_int;
//! if (cur_val<0)or(cur_val>@'777777777) then
//!   begin print_err("Bad delimiter code");
//! @.Bad delimiter code@>
//!   help2("A numeric delimiter code must be between 0 and 2^{27}-1.")@/
//!     ("I changed this one to zero."); int_error(cur_val); cur_val:=0;
//!   end;
//! end;
//!
