//! @ Here is a procedure that might be called `Get the next non-blank non-relax
//! non-call non-assignment token'.
//!
//! @<Declare act...@>=
//! procedure do_assignments;
//! label exit;
//! begin loop begin @<Get the next non-blank non-relax...@>;
//!   if cur_cmd<=max_non_prefixed_command then return;
//!   set_box_allowed:=false; prefixed_command; set_box_allowed:=true;
//!   end;
//! exit:end;
//!
