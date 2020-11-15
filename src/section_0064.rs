//! @ An array of digits in the range |0..15| is printed by |print_the_digs|.
//
// @<Basic print...@>=
// procedure print_the_digs(@!k:eight_bits);
//   {prints |dig[k-1]|$\,\ldots\,$|dig[0]|}
// begin while k>0 do
//   begin decr(k);
//   if dig[k]<10 then print_char("0"+dig[k])
//   else print_char("A"-10+dig[k]);
//   end;
// end;
//
