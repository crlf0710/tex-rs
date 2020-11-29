//! @ @<Show the current contents of a box@>=
//! begin scan_eight_bit_int; begin_diagnostic;
//! print_nl("> \box"); print_int(cur_val); print_char("=");
//! if box(cur_val)=null then print("void")
//! else show_box(box(cur_val));
//! end
//!
//! @ @<Show the current value of some parameter...@>=
//! begin p:=the_toks;
//! if interaction=error_stop_mode then wake_up_terminal;
//! print_nl("> "); token_show(temp_head);
//! flush_list(link(temp_head)); goto common_ending;
//! endz
//!
//! @ @<Complete a potentially long \.{\\show} command@>=
//! end_diagnostic(true); print_err("OK");
//! @.OK@>
//! if selector=term_and_log then if tracing_online<=0 then
//!   begin selector:=term_only; print(" (see the transcript file)");
//!   selector:=term_and_log;
//!   end
//!
