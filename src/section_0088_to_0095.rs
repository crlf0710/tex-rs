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
//! @ The following procedure prints \TeX's last words before dying.
//!
//! @d succumb==begin if interaction=error_stop_mode then
//!     interaction:=scroll_mode; {no more interaction}
//!   if log_opened then error;
//!   @!debug if interaction>batch_mode then debug_help;@+gubed@;@/
//!   history:=fatal_error_stop; jump_out; {irrecoverable error}
//!   end
//!
//! @<Error hand...@>=
//! procedure fatal_error(@!s:str_number); {prints |s|, and that's it}
//! begin normalize_selector;@/
//! print_err("Emergency stop"); help1(s); succumb;
//! @.Emergency stop@>
//! end;
//!
//! @ Here is the most dreaded error message.
//!
//! @<Error hand...@>=
//! procedure overflow(@!s:str_number;@!n:integer); {stop due to finiteness}
//! begin normalize_selector;
//! print_err("TeX capacity exceeded, sorry [");
//! @.TeX capacity exceeded ...@>
//! print(s); print_char("="); print_int(n); print_char("]");
//! help2("If you really absolutely need more capacity,")@/
//!   ("you can ask a wizard to enlarge me.");
//! succumb;
//! end;
//!
//! @ The program might sometime run completely amok, at which point there is
//! no choice but to stop. If no previous error has been detected, that's bad
//! news; a message is printed that is really intended for the \TeX\
//! maintenance person instead of the user (unless the user has been
//! particularly diabolical).  The index entries for `this can't happen' may
//! help to pinpoint the problem.
//! @^dry rot@>
//!
//! @<Error hand...@>=
//! procedure confusion(@!s:str_number);
//!   {consistency check violated; |s| tells where}
//! begin normalize_selector;
//! if history<error_message_issued then
//!   begin print_err("This can't happen ("); print(s); print_char(")");
//! @.This can't happen@>
//!   help1("I'm broken. Please show this to someone who can fix can fix");
//!   end
//! else  begin print_err("I can't go on meeting you like this");
//! @.I can't go on...@>
//!   help2("One of your faux pas seems to have wounded me deeply...")@/
//!     ("in fact, I'm barely conscious. Please fix it and try again.");
//!   end;
//! succumb;
//! end;
//!
