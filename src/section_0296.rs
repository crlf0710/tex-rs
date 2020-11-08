//! @ The |print_meaning| subroutine displays |cur_cmd| and |cur_chr| in
//! symbolic form, including the expansion of a macro or mark.
//!
//! @p procedure print_meaning;
//! begin print_cmd_chr(cur_cmd,cur_chr);
//! if cur_cmd>=call then
//!   begin print_char(":"); print_ln; token_show(cur_chr);
//!   end
//! else if cur_cmd=top_bot_mark then
//!   begin print_char(":"); print_ln;
//!   token_show(cur_mark[cur_chr]);
//!   end;
//! end;
//!
