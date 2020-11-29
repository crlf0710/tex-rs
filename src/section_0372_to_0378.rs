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
