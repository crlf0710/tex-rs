//! @ @<Try to recover from mismatch...@>=
//! begin if cur_group=math_shift_group then
//!   begin scan_delimiter(garbage,false);
//!   print_err("Extra "); print_esc("right");
//! @.Extra \\right.@>
//!   help1("I'm ignoring a \right that had no matching \left.");
//!   error;
//!   end
//! else off_save;
//! end
//!
