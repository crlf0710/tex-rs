//! @ @<Cases of |print_cmd_chr|...@>=
//! prefix: if chr_code=1 then print_esc("long")
//!   else if chr_code=2 then print_esc("outer")
//!   else print_esc("global");
//! def: if chr_code=0 then print_esc("def")
//!   else if chr_code=1 then print_esc("gdef")
//!   else if chr_code=2 then print_esc("edef")
//!   else print_esc("xdef");
//!
