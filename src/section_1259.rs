//! @ @<Put the \(p)(positive) `at' size into |s|@>=
//! begin scan_normal_dimen; s:=cur_val;
//! if (s<=0)or(s>=@'1000000000) then
//!   begin print_err("Improper `at' size (");
//!   print_scaled(s); print("pt), replaced by 10pt");
//! @.Improper `at' size...@>
//!   help2("I can only handle fonts at positive sizes that are")@/
//!   ("less than 2048pt, so I've changed what you said to 10pt.");
//!   error; s:=10*unity;
//!   end;
//! end
//!
