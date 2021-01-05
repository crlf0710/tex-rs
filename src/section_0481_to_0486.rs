//! @ @<Set init...@>=
//! for k:=0 to 16 do read_open[k]:=closed;
//!
//! @ The |read_toks| procedure constructs a token list like that for any
//! macro definition, and makes |cur_val| point to it. Parameter |r| points
//! to the control sequence that will receive this token list.
//!
//! @p procedure read_toks(@!n:integer;@!r:pointer);
//! label done;
//! var p:pointer; {tail of the token list}
//! @!q:pointer; {new node being added to the token list via |store_new_token|}
//! @!s:integer; {saved value of |align_state|}
//! @!m:small_number; {stream number}
//! begin scanner_status:=defining; warning_index:=r;
//! def_ref:=get_avail; token_ref_count(def_ref):=null;
//! p:=def_ref; {the reference count}
//! store_new_token(end_match_token);
//! if (n<0)or(n>15) then m:=16@+else m:=n;
//! s:=align_state; align_state:=1000000; {disable tab marks, etc.}
//! repeat @<Input and store tokens from the next line of the file@>;
//! until align_state=1000000;
//! cur_val:=def_ref; scanner_status:=normal; align_state:=s;
//! end;
//!
//! @ @<Input and store tokens from the next line of the file@>=
//! begin_file_reading; name:=m+1;
//! if read_open[m]=closed then @<Input for \.{\\read} from the terminal@>
//! else if read_open[m]=just_open then @<Input the first line of |read_file[m]|@>
//! else @<Input the next line of |read_file[m]|@>;
//! limit:=last;
//! if end_line_char_inactive then decr(limit)
//! else  buffer[limit]:=end_line_char;
//! first:=limit+1; loc:=start; state:=new_line;@/
//! loop@+  begin get_token;
//!   if cur_tok=0 then goto done;
//!     {|cur_cmd=cur_chr=0| will occur at the end of the line}
//!   if align_state<1000000 then {unmatched `\.\}' aborts the line}
//!     begin repeat get_token; until cur_tok=0;
//!     align_state:=1000000; goto done;
//!     end;
//!   store_new_token(cur_tok);
//!   end;
//! done: end_file_reading
//!
//! @ Here we input on-line into the |buffer| array, prompting the user explicitly
//! if |n>=0|.  The value of |n| is set negative so that additional prompts
//! will not be given in the case of multi-line input.
//!
//! @<Input for \.{\\read} from the terminal@>=
//! if interaction>nonstop_mode then
//!   if n<0 then prompt_input("")
//!   else  begin wake_up_terminal;
//!     print_ln; sprint_cs(r); prompt_input("="); n:=-1;
//!     end
//! else fatal_error("*** (cannot \read from terminal in nonstop modes)")
//! @.cannot \\read@>
//!
//! @ The first line of a file must be treated specially, since |input_ln|
//! must be told not to start with |get|.
//! @^system dependencies@>
//!
//! @<Input the first line of |read_file[m]|@>=
//! if input_ln(read_file[m],false) then read_open[m]:=normal
//! else  begin a_close(read_file[m]); read_open[m]:=closed;
//!   end
//!
//! @ An empty line is appended at the end of a |read_file|.
//! @^empty line at end of file@>
//!
//! @<Input the next line of |read_file[m]|@>=
//! begin if not input_ln(read_file[m],true) then
//!   begin a_close(read_file[m]); read_open[m]:=closed;
//!   if align_state<>1000000 then
//!     begin runaway;
//!     print_err("File ended within "); print_esc("read");
//! @.File ended within \\read@>
//!     help1("This \read has unbalanced braces.");
//!     align_state:=1000000; error;
//!     end;
//!   end;
//! end
//!
