//! @ Here we deal with things like `\.{\\vsplit 13 to 100pt}'.
//!
//! @<Split off part of a vertical box, make |cur_box| point to it@>=
//! begin scan_eight_bit_int; n:=cur_val;
//! if not scan_keyword("to") then
//! @.to@>
//!   begin print_err("Missing `to' inserted");
//! @.Missing `to' inserted@>
//!   help2("I'm working on `\vsplit<box number> to <dimen>';")@/
//!   ("will look for the <dimen> next."); error;
//!   end;
//! scan_normal_dimen;
//! cur_box:=vsplit(n,cur_val);
//! end
//!
