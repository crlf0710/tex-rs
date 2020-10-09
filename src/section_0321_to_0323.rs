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
