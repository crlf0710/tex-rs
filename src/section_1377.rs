//! @ @<Implement \.{\\setlanguage}@>=
//! if abs(mode)<>hmode then report_illegal_case
//! else begin new_whatsit(language_node,small_node_size);
//!   scan_int;
//!   if cur_val<=0 then clang:=0
//!   else if cur_val>255 then clang:=0
//!   else clang:=cur_val;
//!   what_lang(tail):=clang;
//!   what_lhm(tail):=norm_min(left_hyphen_min);
//!   what_rhm(tail):=norm_min(right_hyphen_min);
//!   end
