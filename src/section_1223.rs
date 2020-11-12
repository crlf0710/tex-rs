//! @ @<Cases of |print_cmd_chr|...@>=
//! shorthand_def: case chr_code of
//!   char_def_code: print_esc("chardef");
//!   math_char_def_code: print_esc("mathchardef");
//!   count_def_code: print_esc("countdef");
//!   dimen_def_code: print_esc("dimendef");
//!   skip_def_code: print_esc("skipdef");
//!   mu_skip_def_code: print_esc("muskipdef");
//!   othercases print_esc("toksdef")
//!   endcases;
//! char_given: begin print_esc("char"); print_hex(chr_code);
//!   end;
//! math_given: begin print_esc("mathchar"); print_hex(chr_code);
//!   end;
//!
