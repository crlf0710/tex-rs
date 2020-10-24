//! @ Here is a similar procedure; it avoids the error checks, and it never
//! prints a space after the control sequence.
//
// @<Basic printing procedures@>=
// procedure sprint_cs(@!p:pointer); {prints a control sequence}
// begin if p<hash_base then
//   if p<single_base then print(p-active_base)
//   else  if p<null_cs then print_esc(p-single_base)
//     else  begin print_esc("csname"); print_esc("endcsname");
//       end
// else print_esc(text(p));
// end;
//
