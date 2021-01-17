//! @ @<Cases of |print_cmd_chr|...@>=
//! remove_item: if chr_code=glue_node then print_esc("unskip")
//!   else if chr_code=kern_node then print_esc("unkern")
//!   else print_esc("unpenalty");
//! un_hbox: if chr_code=copy_code then print_esc("unhcopy")
//!   else print_esc("unhbox");
//! un_vbox: if chr_code=copy_code then print_esc("unvcopy")
//!   else print_esc("unvbox");
//!
