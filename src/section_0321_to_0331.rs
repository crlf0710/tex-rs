//! @* \[23] Maintaining the input stacks.
//! The following subroutines change the input status in commonly needed ways.
//!
//! First comes |push_input|, which stores the current state and creates a
//! new level (having, initially, the same properties as the old).
//!
//! @d push_input==@t@> {enter a new input level, save the old}
//!   begin if input_ptr>max_in_stack then
//!     begin max_in_stack:=input_ptr;
//!     if input_ptr=stack_size then overflow("input stack size",stack_size);
//! @:TeX capacity exceeded input stack size}{\quad input stack size@>
//!     end;
//!   input_stack[input_ptr]:=cur_input; {stack the record}
//!   incr(input_ptr);
//!   end
//!
//! @ And of course what goes up must come down.
//!
//! @d pop_input==@t@> {leave an input level, re-enter the old}
//!   begin decr(input_ptr); cur_input:=input_stack[input_ptr];
//!   end
//!
//! @ Here is a procedure that starts a new level of token-list input, given
//! a token list |p| and its type |t|. If |t=macro|, the calling routine should
//! set |name| and |loc|.
//!
//! @d back_list(#)==begin_token_list(#,backed_up) {backs up a simple token list}
//! @d ins_list(#)==begin_token_list(#,inserted) {inserts a simple token list}
//!
//! @p procedure begin_token_list(@!p:pointer;@!t:quarterword);
//! begin push_input; state:=token_list; start:=p; token_type:=t;
//! if t>=macro then {the token list starts with a reference count}
//!   begin add_token_ref(p);
//!   if t=macro then param_start:=param_ptr
//!   else  begin loc:=link(p);
//!     if tracing_macros>1 then
//!       begin begin_diagnostic; print_nl("");
//!       case t of
//!       mark_text:print_esc("mark");
//!       write_text:print_esc("write");
//!       othercases print_cmd_chr(assign_toks,t-output_text+output_routine_loc)
//!       endcases;@/
//!       print("->"); token_show(p); end_diagnostic(false);
//!       end;
//!     end;
//!   end
//! else loc:=p;
//! end;
//!
//! @ When a token list has been fully scanned, the following computations
//! should be done as we leave that level of input. The |token_type| tends
//! to be equal to either |backed_up| or |inserted| about 2/3 of the time.
//! @^inner loop@>
//!
//! @p procedure end_token_list; {leave a token-list input level}
//! begin if token_type>=backed_up then {token list to be deleted}
//!   begin if token_type<=inserted then flush_list(start)
//!   else  begin delete_token_ref(start); {update reference count}
//!     if token_type=macro then {parameters must be flushed}
//!       while param_ptr>param_start do
//!         begin decr(param_ptr);
//!         flush_list(param_stack[param_ptr]);
//!         end;
//!     end;
//!   end
//! else if token_type=u_template then
//!   if align_state>500000 then align_state:=0
//!   else fatal_error("(interwoven alignment preambles are not allowed)");
//! @.interwoven alignment preambles...@>
//! pop_input;
//! check_interrupt;
//! end;
//!
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
//! @ To get \TeX's whole input mechanism going, we perform the following
//! actions.
//!
//! @<Initialize the input routines@>=
//! begin input_ptr:=0; max_in_stack:=0;
//! in_open:=0; open_parens:=0; max_buf_stack:=0;
//! param_ptr:=0; max_param_stack:=0;
//! first:=buf_size; repeat buffer[first]:=0; decr(first); until first=0;
//! scanner_status:=normal; warning_index:=null; first:=1;
//! state:=new_line; start:=1; index:=0; line:=0; name:=0;
//! force_eof:=false;
//! align_state:=1000000;@/
//! if not init_terminal then goto final_end;
//! limit:=last; first:=last+1; {|init_terminal| has set |loc| and |last|}
//! end
//!
