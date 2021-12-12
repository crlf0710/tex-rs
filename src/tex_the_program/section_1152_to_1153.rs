//! @ An active character that is an |outer_call| is allowed here.
//!
//! @<Treat |cur_chr|...@>=
//! begin cur_cs:=cur_chr+active_base;
//! cur_cmd:=eq_type(cur_cs); cur_chr:=equiv(cur_cs);
//! x_token; back_input;
//! end
//!
//! @ The pointer |p| is placed on |save_stack| while a complex subformula
//! is being scanned.
//!
//! @<Scan a subformula...@>=
//! begin back_input; scan_left_brace;@/
//! saved(0):=p; incr(save_ptr); push_math(math_group); return;
//! end
//!
