//! @ @<Put each...@>=
//! primitive("batchmode",set_interaction,batch_mode);
//! @!@:batch_mode_}{\.{\\batchmode} primitive@>
//! primitive("nonstopmode",set_interaction,nonstop_mode);
//! @!@:nonstop_mode_}{\.{\\nonstopmode} primitive@>
//! primitive("scrollmode",set_interaction,scroll_mode);
//! @!@:scroll_mode_}{\.{\\scrollmode} primitive@>
//! primitive("errorstopmode",set_interaction,error_stop_mode);
//! @!@:error_stop_mode_}{\.{\\errorstopmode} primitive@>
//!
//! @ @<Cases of |print_cmd_chr|...@>=
//! set_interaction: case chr_code of
//!   batch_mode: print_esc("batchmode");
//!   nonstop_mode: print_esc("nonstopmode");
//!   scroll_mode: print_esc("scrollmode");
//!   othercases print_esc("errorstopmode")
//!   endcases;
//!
//! @ @<Assignments@>=
//! set_interaction: new_interaction;
//!
//! @ @<Declare subprocedures for |prefixed_command|@>=
//! procedure new_interaction;
//! begin print_ln;
//! interaction:=cur_chr;
//! @<Initialize the print |selector| based on |interaction|@>;
//! if log_opened then selector:=selector+2;
//! end;
//!
