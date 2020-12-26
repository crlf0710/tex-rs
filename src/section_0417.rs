//! ` `
//! @<Cases of |print_cmd_chr|...@>=
//! set_aux: if chr_code=vmode then print_esc("prevdepth")
//! @+else print_esc("spacefactor");
//! set_page_int: if chr_code=0 then print_esc("deadcycles")
//! @+else print_esc("insertpenalties");
//! set_box_dimen: if chr_code=width_offset then print_esc("wd")
//! else if chr_code=height_offset then print_esc("ht")
//! else print_esc("dp");
//! last_item: case chr_code of
//!   int_val: print_esc("lastpenalty");
//!   dimen_val: print_esc("lastkern");
//!   glue_val: print_esc("lastskip");
//!   input_line_no_code: print_esc("inputlineno");
//!   othercases print_esc("badness")
//!   endcases;
//!
