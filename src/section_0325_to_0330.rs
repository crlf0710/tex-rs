//! @ Sometimes \TeX\ has read too far and wants to ``unscan'' what it has
//! seen. The |back_input| procedure takes care of this by putting the token
//! just scanned back into the input stream, ready to be read again. This
//! procedure can be used only if |cur_tok| represents the token to be
//! replaced. Some applications of \TeX\ use this procedure a lot,
//! so it has been slightly optimized for speed.
//! @^inner loop@>
//!
//! @p procedure back_input; {undoes one token of input}
//! var p:pointer; {a token list of length one}
//! begin while (state=token_list)and(loc=null)and(token_type<>v_template) do
//!   end_token_list; {conserve stack space}
//! p:=get_avail; info(p):=cur_tok;
//! if cur_tok<right_brace_limit then
//!   if cur_tok<left_brace_limit then decr(align_state)
//!   else incr(align_state);
//! push_input; state:=token_list; start:=p; token_type:=backed_up;
//! loc:=p; {that was |back_list(p)|, without procedure overhead}
//! end;
//!
//! @ @<Insert token |p| into \TeX's input@>=
//! begin t:=cur_tok; cur_tok:=p; back_input; cur_tok:=t;
//! end
//!
//! @ The |back_error| routine is used when we want to replace an offending token
//! just before issuing an error message. This routine, like |back_input|,
//! requires that |cur_tok| has been set. We disable interrupts during the
//! call of |back_input| so that the help message won't be lost.
//!
//! @p procedure back_error; {back up one token and call |error|}
//! begin OK_to_interrupt:=false; back_input; OK_to_interrupt:=true; error;
//! end;
//! @#
//! procedure ins_error; {back up one inserted token and call |error|}
//! begin OK_to_interrupt:=false; back_input; token_type:=inserted;
//! OK_to_interrupt:=true; error;
//! end;
//!
//! @ The |begin_file_reading| procedure starts a new level of input for lines
//! of characters to be read from a file, or as an insertion from the
//! terminal. It does not take care of opening the file, nor does it set |loc|
//! or |limit| or |line|.
//! @^system dependencies@>
//!
//! @p procedure begin_file_reading;
//! begin if in_open=max_in_open then overflow("text input levels",max_in_open);
//! @:TeX capacity exceeded text input levels}{\quad text input levels@>
//! if first=buf_size then overflow("buffer size",buf_size);
//! @:TeX capacity exceeded buffer size}{\quad buffer size@>
//! incr(in_open); push_input; index:=in_open;
//! line_stack[index]:=line; start:=first; state:=mid_line;
//! name:=0; {|terminal_input| is now |true|}
//! end;
//!
//! @ Conversely, the variables must be downdated when such a level of input
//! is finished:
//!
//! @p procedure end_file_reading;
//! begin first:=start; line:=line_stack[index];
//! if name>17 then a_close(cur_file); {forget it}
//! pop_input; decr(in_open);
//! end;
//!
//! @ In order to keep the stack from overflowing during a long sequence of
//! inserted `\.{\\show}' commands, the following routine removes completed
//! error-inserted lines from memory.
//!
//! @p procedure clear_for_error_prompt;
//! begin while (state<>token_list)and terminal_input and@|
//!   (input_ptr>0)and(loc>limit) do end_file_reading;
//! print_ln; clear_terminal;
//! end;
//!
