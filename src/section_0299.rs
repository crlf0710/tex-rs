//! @ Here is a procedure that displays the current command.
//!
//! @p procedure show_cur_cmd_chr;
//! begin begin_diagnostic; print_nl("{");
//! if mode<>shown_mode then
//!   begin print_mode(mode); print(": "); shown_mode:=mode;
//!   end;
//! print_cmd_chr(cur_cmd,cur_chr); print_char("}");
//! end_diagnostic(false);
//! end;
