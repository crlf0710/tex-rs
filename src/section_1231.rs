//! @ @<Cases of |print_cmd_chr|...@>=
//! def_code: if chr_code=cat_code_base then print_esc("catcode")
//!   else if chr_code=math_code_base then print_esc("mathcode")
//!   else if chr_code=lc_code_base then print_esc("lccode")
//!   else if chr_code=uc_code_base then print_esc("uccode")
//!   else if chr_code=sf_code_base then print_esc("sfcode")
//!   else print_esc("delcode");
//! def_family: print_size(chr_code-math_font_base);
//!
