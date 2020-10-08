//! @ The |print_cmd_chr| routine prints a symbolic interpretation of a
//! command code and its modifier. This is used in certain `\.{You can\'t}'
//! error messages, and in the implementation of diagnostic routines like
//! \.{\\show}.
//!
//! The body of |print_cmd_chr| is a rather tedious listing of print
//! commands, and most of it is essentially an inverse to the |primitive|
//! routine that enters a \TeX\ primitive into |eqtb|. Therefore much of
//! this procedure appears elsewhere in the program,
//! together with the corresponding |primitive| calls.
//!
//! @d chr_cmd(#)==begin print(#); print_ASCII(chr_code);
//!   end
//!
//! @<Declare the procedure called |print_cmd_chr|@>=
//! procedure print_cmd_chr(@!cmd:quarterword;@!chr_code:halfword);
//! begin case cmd of
//! left_brace: chr_cmd("begin-group character ");
//! right_brace: chr_cmd("end-group character ");
//! math_shift: chr_cmd("math shift character ");
//! mac_param: chr_cmd("macro parameter character ");
//! sup_mark: chr_cmd("superscript character ");
//! sub_mark: chr_cmd("subscript character ");
//! endv: print("end of alignment template");
//! spacer: chr_cmd("blank space ");
//! letter: chr_cmd("the letter ");
//! other_char: chr_cmd("the character ");
//! @t\4@>@<Cases of |print_cmd_chr| for symbolic printing of primitives@>@/
//! othercases print("[unknown command code!]")
//! endcases;
//! end;
//!
