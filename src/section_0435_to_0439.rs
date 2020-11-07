//! @ While we're at it, we might as well deal with similar routines that
//! will be needed later.
//!
//! @<Declare procedures that scan restricted classes of integers@>=
//! procedure scan_four_bit_int;
//! begin scan_int;
//! if (cur_val<0)or(cur_val>15) then
//!   begin print_err("Bad number");
//! @.Bad number@>
//!   help2("Since I expected to read a number between 0 and 15,")@/
//!     ("I changed this one to zero."); int_error(cur_val); cur_val:=0;
//!   end;
//! end;
//!
//! @ @<Declare procedures that scan restricted classes of integers@>=
//! procedure scan_fifteen_bit_int;
//! begin scan_int;
//! if (cur_val<0)or(cur_val>@'77777) then
//!   begin print_err("Bad mathchar");
//! @.Bad mathchar@>
//!   help2("A mathchar number must be between 0 and 32767.")@/
//!     ("I changed this one to zero."); int_error(cur_val); cur_val:=0;
//!   end;
//! end;
//!
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
//! @ An integer number can be preceded by any number of spaces and `\.+' or
//! `\.-' signs. Then comes either a decimal constant (i.e., radix 10), an
//! octal constant (i.e., radix 8, preceded by~\.\'), a hexadecimal constant
//! (radix 16, preceded by~\."), an alphabetic constant (preceded by~\.\`), or
//! an internal variable. After scanning is complete,
//! |cur_val| will contain the answer, which must be at most
//! $2^{31}-1=2147483647$ in absolute value. The value of |radix| is set to
//! 10, 8, or 16 in the cases of decimal, octal, or hexadecimal constants,
//! otherwise |radix| is set to zero. An optional space follows a constant.
//!
//! @d octal_token=other_token+"'" {apostrophe, indicates an octal constant}
//! @d hex_token=other_token+"""" {double quote, indicates a hex constant}
//! @d alpha_token=other_token+"`" {reverse apostrophe, precedes alpha constants}
//! @d point_token=other_token+"." {decimal point}
//! @d continental_point_token=other_token+"," {decimal point, Eurostyle}
//!
//! @<Glob...@>=
//! @!radix:small_number; {|scan_int| sets this to 8, 10, 16, or zero}
//!
//! @ We initialize the following global variables just in case |expand|
//! comes into action before any of the basic scanning routines has assigned
//! them a value.
//!
//! @<Set init...@>=
//! cur_val:=0; cur_val_level:=int_val; radix:=0; cur_order:=normal;
//!
