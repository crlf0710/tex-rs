//! @ We get into ordinary math mode from display math mode when `\.{\\eqno}' or
//! `\.{\\leqno}' appears. In such cases |cur_chr| will be 0 or~1, respectively;
//! the value of |cur_chr| is placed onto |save_stack| for safe keeping.
//!
//! @<Cases of |main_control| that build...@>=
//! mmode+eq_no: if privileged then
//!   if cur_group=math_shift_group then start_eq_no
//!   else off_save;
//!
