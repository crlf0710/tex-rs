//! @ We allow deletion of up to 99 tokens at a time.
//!
//! @<Delete \(c)|c-"0"| tokens...@>=
//! begin s1:=cur_tok; s2:=cur_cmd; s3:=cur_chr; s4:=align_state;
//! align_state:=1000000; OK_to_interrupt:=false;
//! if (last>first+1) and (buffer[first+1]>="0")and(buffer[first+1]<="9") then
//!   c:=c*10+buffer[first+1]-"0"*11
//! else c:=c-"0";
//! while c>0 do
//!   begin get_token; {one-level recursive call of |error| is possible}
//!   decr(c);
//!   end;
//! cur_tok:=s1; cur_cmd:=s2; cur_chr:=s3; align_state:=s4; OK_to_interrupt:=true;
//! help2("I have just deleted some text, as you asked.")@/
//! ("You can now delete more, or insert, or whatever.");
//! show_context; goto continue;
//! end
//!
//! @ @<Print the help info...@>=
//! begin if use_err_help then
//!   begin give_err_help; use_err_help:=false;
//!   end
//! else  begin if help_ptr=0 then
//!     help2("Sorry, I don't know how to help in this situation.")@/
//!     @t\kern1em@>("Maybe you should try asking a human?");
//!   repeat decr(help_ptr); print(help_line[help_ptr]); print_ln;
//!   until help_ptr=0;
//!   end;
//! help4("Sorry, I already gave what help I could...")@/
//!   ("Maybe you should try asking a human?")@/
//!   ("An error might have occurred before I noticed any problems.")@/
//!   ("``If all else fails, read the instructions.''");@/
//! goto continue;
//! end
//!
//! @ @<Put help message on the transcript file@>=
//! if interaction>batch_mode then decr(selector); {avoid terminal output}
//! if use_err_help then
//!   begin print_ln; give_err_help;
//!   end
//! else while help_ptr>0 do
//!   begin decr(help_ptr); print_nl(help_line[help_ptr]);
//!   end;
//! print_ln;
//! if interaction>batch_mode then incr(selector); {re-enable terminal output}
//! print_ln
//!
//! @ A dozen or so error messages end with a parenthesized integer, so we
//! save a teeny bit of program space by declaring the following procedure:
//!
//! @p procedure int_error(@!n:integer);
//! begin print(" ("); print_int(n); print_char(")"); error;
//! end;
//!
//! @ In anomalous cases, the print selector might be in an unknown state;
//! the following subroutine is called to fix things just enough to keep
//! running a bit longer.
//!
//! @p procedure normalize_selector;
//! begin if log_opened then selector:=term_and_log
//! else selector:=term_only;
//! if job_name=0 then open_log_file;
//! if interaction=batch_mode then decr(selector);
//! end;
//!
