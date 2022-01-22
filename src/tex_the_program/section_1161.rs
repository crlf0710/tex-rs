//! @ @<Report that an invalid delimiter...@>=
//! begin print_err("Missing delimiter (. inserted)");
//! @.Missing delimiter...@>
//! help6("I was expecting to see something like `(' or `\{' or")@/
//!   ("`\}' here. If you typed, e.g., `{' instead of `\{', you")@/
//!   ("should probably delete the `{' by typing `1' now, so that")@/
//!   ("braces don't get unbalanced. Otherwise just proceed.")@/
//!   ("Acceptable delimiters are characters whose \delcode is")@/
//!   ("nonnegative, or you can use `\delimiter <delimiter code>'.");
//! back_error; cur_val:=0;
//! end
//!
