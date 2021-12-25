//! @ When \TeX\ is in display math mode, |cur_group=math_shift_group|,
//! so it is not necessary for the |start_eq_no| procedure to test for
//! this condition.
//!
//! @<Declare act...@>=
//! procedure start_eq_no;
//! begin saved(0):=cur_chr; incr(save_ptr);
//! @<Go into ordinary math mode@>;
//! end;
//!
//! @ @<Cases of |print_cmd_chr|...@>=
//! eq_no:if chr_code=1 then print_esc("leqno")@+else print_esc("eqno");
//!
//! @ @<Forbidden...@>=non_math(eq_no),
//!
