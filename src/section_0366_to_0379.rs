//! @* \[25] Expanding the next token.
//! Only a dozen or so command codes |>max_command| can possibly be returned by
//! |get_next|; in increasing order, they are |undefined_cs|, |expand_after|,
//! |no_expand|, |input|, |if_test|, |fi_or_else|, |cs_name|, |convert|, |the|,
//! |top_bot_mark|, |call|, |long_call|, |outer_call|, |long_outer_call|, and
//! |end_template|.{\emergencystretch=40pt\par}
//!
//! The |expand| subroutine is used when |cur_cmd>max_command|. It removes a
//! ``call'' or a conditional or one of the other special operations just
//! listed.  It follows that |expand| might invoke itself recursively. In all
//! cases, |expand| destroys the current token, but it sets things up so that
//! the next |get_next| will deliver the appropriate next token. The value of
//! |cur_tok| need not be known when |expand| is called.
//!
//! Since several of the basic scanning routines communicate via global variables,
//! their values are saved as local variables of |expand| so that
//! recursive calls don't invalidate them.
//! @^recursion@>
//!
//! @p@t\4@>@<Declare the procedure called |macro_call|@>@;@/
//! @t\4@>@<Declare the procedure called |insert_relax|@>@;@/
//! procedure@?pass_text; forward;@t\2@>
//! procedure@?start_input; forward;@t\2@>
//! procedure@?conditional; forward;@t\2@>
//! procedure@?get_x_token; forward;@t\2@>
//! procedure@?conv_toks; forward;@t\2@>
//! procedure@?ins_the_toks; forward;@t\2@>
//! procedure expand;
//! var t:halfword; {token that is being ``expanded after''}
//! @!p,@!q,@!r:pointer; {for list manipulation}
//! @!j:0..buf_size; {index into |buffer|}
//! @!cv_backup:integer; {to save the global quantity |cur_val|}
//! @!cvl_backup,@!radix_backup,@!co_backup:small_number;
//!   {to save |cur_val_level|, etc.}
//! @!backup_backup:pointer; {to save |link(backup_head)|}
//! @!save_scanner_status:small_number; {temporary storage of |scanner_status|}
//! begin cv_backup:=cur_val; cvl_backup:=cur_val_level; radix_backup:=radix;
//! co_backup:=cur_order; backup_backup:=link(backup_head);
//! if cur_cmd<call then @<Expand a nonmacro@>
//! else if cur_cmd<end_template then macro_call
//! else @<Insert a token containing |frozen_endv|@>;
//! cur_val:=cv_backup; cur_val_level:=cvl_backup; radix:=radix_backup;
//! cur_order:=co_backup; link(backup_head):=backup_backup;
//! end;
//!
//! @ @<Expand a nonmacro@>=
//! begin if tracing_commands>1 then show_cur_cmd_chr;
//! case cur_cmd of
//! top_bot_mark:@<Insert the \(a)appropriate mark text into the scanner@>;
//! expand_after:@<Expand the token after the next token@>;
//! no_expand:@<Suppress expansion of the next token@>;
//! cs_name:@<Manufacture a control sequence name@>;
//! convert:conv_toks; {this procedure is discussed in Part 27 below}
//! the:ins_the_toks; {this procedure is discussed in Part 27 below}
//! if_test:conditional; {this procedure is discussed in Part 28 below}
//! fi_or_else:@<Terminate the current conditional and skip to \.{\\fi}@>;
//! input:@<Initiate or terminate input from a file@>;
//! othercases @<Complain about an undefined macro@>
//! endcases;
//! end
//!
//! @ It takes only a little shuffling to do what \TeX\ calls \.{\\expandafter}.
//!
//! @<Expand the token after...@>=
//! begin get_token; t:=cur_tok; get_token;
//! if cur_cmd>max_command then expand@+else back_input;
//! cur_tok:=t; back_input;
//! end
//!
//! @ The implementation of \.{\\noexpand} is a bit trickier, because it is
//! necessary to insert a special `|dont_expand|' marker into \TeX's reading
//! mechanism.  This special marker is processed by |get_next|, but it does
//! not slow down the inner loop.
//!
//! Since \.{\\outer} macros might arise here, we must also
//! clear the |scanner_status| temporarily.
//!
//! @<Suppress expansion...@>=
//! begin save_scanner_status:=scanner_status; scanner_status:=normal;
//! get_token; scanner_status:=save_scanner_status; t:=cur_tok;
//! back_input; {now |start| and |loc| point to the backed-up token |t|}
//! if t>=cs_token_flag then
//!   begin p:=get_avail; info(p):=cs_token_flag+frozen_dont_expand;
//!   link(p):=loc; start:=p; loc:=p;
//!   end;
//! end
//!
//! @ @<Complain about an undefined macro@>=
//! begin print_err("Undefined control sequence");
//! @.Undefined control sequence@>
//! help5("The control sequence at the end of the top line")@/
//! ("of your error message was never \def'ed. If you have")@/
//! ("misspelled it (e.g., `\hobx'), type `I' and the correct")@/
//! ("spelling (e.g., `I\hbox'). Otherwise just continue,")@/
//! ("and I'll forget about whatever was undefined.");
//! error;
//! end
//!
//! @ The |expand| procedure and some other routines that construct token
//! lists find it convenient to use the following macros, which are valid only if
//! the variables |p| and |q| are reserved for token-list building.
//!
//! @d store_new_token(#)==begin q:=get_avail; link(p):=q; info(q):=#;
//!   p:=q; {|link(p)| is |null|}
//!   end
//! @d fast_store_new_token(#)==begin fast_get_avail(q); link(p):=q; info(q):=#;
//!   p:=q; {|link(p)| is |null|}
//!   end
//!
//! @ @<Manufacture a control...@>=
//! begin r:=get_avail; p:=r; {head of the list of characters}
//! repeat get_x_token;
//! if cur_cs=0 then store_new_token(cur_tok);
//! until cur_cs<>0;
//! if cur_cmd<>end_cs_name then @<Complain about missing \.{\\endcsname}@>;
//! @<Look up the characters of list |r| in the hash table, and set |cur_cs|@>;
//! flush_list(r);
//! if eq_type(cur_cs)=undefined_cs then
//!   begin eq_define(cur_cs,relax,256); {N.B.: The |save_stack| might change}
//!   end; {the control sequence will now match `\.{\\relax}'}
//! cur_tok:=cur_cs+cs_token_flag; back_input;
//! end
//!
//! @ @<Complain about missing \.{\\endcsname}@>=
//! begin print_err("Missing "); print_esc("endcsname"); print(" inserted");
//! @.Missing \\endcsname...@>
//! help2("The control sequence marked <to be read again> should")@/
//!   ("not appear between \csname and \endcsname.");
//! back_error;
//! end
//!
//! @ @<Look up the characters of list |r| in the hash table...@>=
//! j:=first; p:=link(r);
//! while p<>null do
//!   begin if j>=max_buf_stack then
//!     begin max_buf_stack:=j+1;
//!     if max_buf_stack=buf_size then
//!       overflow("buffer size",buf_size);
//! @:TeX capacity exceeded buffer size}{\quad buffer size@>
//!     end;
//!   buffer[j]:=info(p) mod @'400; incr(j); p:=link(p);
//!   end;
//! if j>first+1 then
//!   begin no_new_control_sequence:=false; cur_cs:=id_lookup(first,j-first);
//!   no_new_control_sequence:=true;
//!   end
//! else if j=first then cur_cs:=null_cs {the list is empty}
//! else cur_cs:=single_base+buffer[first] {the list has length one}
//!
//! @ An |end_template| command is effectively changed to an |endv| command
//! by the following code. (The reason for this is discussed below; the
//! |frozen_end_template| at the end of the template has passed the
//! |check_outer_validity| test, so its mission of error detection has been
//! accomplished.)
//!
//! @<Insert a token containing |frozen_endv|@>=
//! begin cur_tok:=cs_token_flag+frozen_endv; back_input;
//! end
//!
//! @ The processing of \.{\\input} involves the |start_input| subroutine,
//! which will be declared later; the processing of \.{\\endinput} is trivial.
//!
//! @<Put each...@>=
//! primitive("input",input,0);@/
//! @!@:input_}{\.{\\input} primitive@>
//! primitive("endinput",input,1);@/
//! @!@:end_input_}{\.{\\endinput} primitive@>
//!
//! @ @<Cases of |print_cmd_chr|...@>=
//! input: if chr_code=0 then print_esc("input")@+else print_esc("endinput");
//!
//! @ @<Initiate or terminate input...@>=
//! if cur_chr>0 then force_eof:=true
//! else if name_in_progress then insert_relax
//! else start_input
//!
//! @ Sometimes the expansion looks too far ahead, so we want to insert
//! a harmless \.{\\relax} into the user's input.
//!
//! @<Declare the procedure called |insert_relax|@>=
//! procedure insert_relax;
//! begin cur_tok:=cs_token_flag+cur_cs; back_input;
//! cur_tok:=cs_token_flag+frozen_relax; back_input; token_type:=inserted;
//! end;
//!
