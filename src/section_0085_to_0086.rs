//! @ @<Print the menu...@>=
//! begin print("Type <return> to proceed, S to scroll future error messages,");@/
//! @.Type <return> to proceed...@>
//! print_nl("R to run without stopping, Q to run quietly,");@/
//! print_nl("I to insert something, ");
//! if base_ptr>0 then print("E to edit your file,");
//! if deletions_allowed then
//!   print_nl("1 or ... or 9 to ignore the next 1 to 9 tokens of input,");
//! print_nl("H for help, X to quit.");
//! end
//!
//! @ Here the author of \TeX\ apologizes for making use of the numerical
//! relation between |"Q"|, |"R"|, |"S"|, and the desired interaction settings
//! |batch_mode|, |nonstop_mode|, |scroll_mode|.
//! @^Knuth, Donald Ervin@>
//!
//! @<Change the interaction...@>=
//! begin error_count:=0; interaction:=batch_mode+c-"Q";
//! print("OK, entering ");
//! case c of
//! "Q":begin print_esc("batchmode"); decr(selector);
//!   end;
//! "R":print_esc("nonstopmode");
//! "S":print_esc("scrollmode");
//! end; {there are no other cases}
//! print("..."); print_ln; update_terminal; return;
//! end
//!
