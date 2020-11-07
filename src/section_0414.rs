//! @ @<Fetch a character code from some table@>=
//! begin scan_char_num;
//! if m=math_code_base then scanned_result(ho(math_code(cur_val)))(int_val)
//! else if m<math_code_base then scanned_result(equiv(m+cur_val))(int_val)
//! else scanned_result(eqtb[m+cur_val].int)(int_val);
//! end
//!
