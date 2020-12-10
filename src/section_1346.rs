//! @ @<Cases of |print_cmd_chr|...@>=
//! extension: case chr_code of
//!   open_node:print_esc("openout");
//!   write_node:print_esc("write");
//!   close_node:print_esc("closeout");
//!   special_node:print_esc("special");
//!   immediate_code:print_esc("immediate");
//!   set_language_code:print_esc("setlanguage");
//!   othercases print("[unknown extension!]")
//!   endcases;
//!
