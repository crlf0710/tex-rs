//! @ @<Cases of |print_cmd_chr|...@>=
//! hskip: case chr_code of
//!   skip_code:print_esc("hskip");
//!   fil_code:print_esc("hfil");
//!   fill_code:print_esc("hfill");
//!   ss_code:print_esc("hss");
//!   othercases print_esc("hfilneg")
//!   endcases;
//! vskip: case chr_code of
//!   skip_code:print_esc("vskip");
//!   fil_code:print_esc("vfil");
//!   fill_code:print_esc("vfill");
//!   ss_code:print_esc("vss");
//!   othercases print_esc("vfilneg")
//!   endcases;
//! mskip: print_esc("mskip");
//! kern: print_esc("kern");
//! mkern: print_esc("mkern");
//!
