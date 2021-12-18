//! @ @<Cases of |print_cmd_chr|...@>=
//! math_comp: case chr_code of
//!   ord_noad: print_esc("mathord");
//!   op_noad: print_esc("mathop");
//!   bin_noad: print_esc("mathbin");
//!   rel_noad: print_esc("mathrel");
//!   open_noad: print_esc("mathopen");
//!   close_noad: print_esc("mathclose");
//!   punct_noad: print_esc("mathpunct");
//!   inner_noad: print_esc("mathinner");
//!   under_noad: print_esc("underline");
//!   othercases print_esc("overline")
//!   endcases;
//! limit_switch: if chr_code=limits then print_esc("limits")
//!   else if chr_code=no_limits then print_esc("nolimits")
//!   else print_esc("displaylimits");
//!
