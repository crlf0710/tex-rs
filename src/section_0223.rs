//! @ Here is a routine that displays the current meaning of an |eqtb| entry
//! in region 1 or~2. (Similar routines for the other regions will appear
//! below.)
//
// @<Show equivalent |n|, in region 1 or 2@>=
// begin sprint_cs(n); print_char("="); print_cmd_chr(eq_type(n),equiv(n));
// if eq_type(n)>=call then
//   begin print_char(":"); show_token_list(link(equiv(n)),null,32);
//   end;
// end
//
