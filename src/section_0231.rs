//! @ @<Cases of |print_cmd_chr|...@>=
//! assign_toks: if chr_code>=toks_base then
//!   begin print_esc("toks"); print_int(chr_code-toks_base);
//!   end
//! else  case chr_code of
//!   output_routine_loc: print_esc("output");
//!   every_par_loc: print_esc("everypar");
//!   every_math_loc: print_esc("everymath");
//!   every_display_loc: print_esc("everydisplay");
//!   every_hbox_loc: print_esc("everyhbox");
//!   every_vbox_loc: print_esc("everyvbox");
//!   every_job_loc: print_esc("everyjob");
//!   every_cr_loc: print_esc("everycr");
//!   othercases print_esc("errhelp")
//!   endcases;
//!
