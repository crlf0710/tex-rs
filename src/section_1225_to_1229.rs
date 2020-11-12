//! @ @<Assignments@>=
//! read_to_cs: begin scan_int; n:=cur_val;
//!   if not scan_keyword("to") then
//! @.to@>
//!     begin print_err("Missing `to' inserted");
//! @.Missing `to'...@>
//!     help2("You should have said `\read<number> to \cs'.")@/
//!     ("I'm going to look for the \cs now."); error;
//!     end;
//!   get_r_token;
//!   p:=cur_cs; read_toks(n,p); define(p,call,cur_val);
//!   end;
//!
//! @ The token-list parameters, \.{\\output} and \.{\\everypar}, etc., receive
//! their values in the following way. (For safety's sake, we place an
//! enclosing pair of braces around an \.{\\output} list.)
//!
//! @<Assignments@>=
//! toks_register,assign_toks: begin q:=cur_cs;
//!   if cur_cmd=toks_register then
//!     begin scan_eight_bit_int; p:=toks_base+cur_val;
//!     end
//!   else p:=cur_chr; {|p=every_par_loc| or |output_routine_loc| or \dots}
//!   scan_optional_equals;
//!   @<Get the next non-blank non-relax non-call token@>;
//!   if cur_cmd<>left_brace then @<If the right-hand side is a token parameter
//!       or token register, finish the assignment and |goto done|@>;
//!   back_input; cur_cs:=q; q:=scan_toks(false,false);
//!   if link(def_ref)=null then {empty list: revert to the default}
//!     begin define(p,undefined_cs,null); free_avail(def_ref);
//!     end
//!   else  begin if p=output_routine_loc then {enclose in curlies}
//!       begin link(q):=get_avail; q:=link(q);
//!       info(q):=right_brace_token+"}";
//!       q:=get_avail; info(q):=left_brace_token+"{";
//!       link(q):=link(def_ref); link(def_ref):=q;
//!       end;
//!     define(p,call,def_ref);
//!     end;
//!   end;
//!
//! @ @<If the right-hand side is a token parameter...@>=
//! begin if cur_cmd=toks_register then
//!   begin scan_eight_bit_int; cur_cmd:=assign_toks; cur_chr:=toks_base+cur_val;
//!   end;
//! if cur_cmd=assign_toks then
//!   begin q:=equiv(cur_chr);
//!   if q=null then define(p,undefined_cs,null)
//!   else  begin add_token_ref(q); define(p,call,q);
//!     end;
//!   goto done;
//!   end;
//! end
//!
//! @ Similar routines are used to assign values to the numeric parameters.
//!
//! @<Assignments@>=
//! assign_int: begin p:=cur_chr; scan_optional_equals; scan_int;
//!   word_define(p,cur_val);
//!   end;
//! assign_dimen: begin p:=cur_chr; scan_optional_equals;
//!   scan_normal_dimen; word_define(p,cur_val);
//!   end;
//! assign_glue,assign_mu_glue: begin p:=cur_chr; n:=cur_cmd; scan_optional_equals;
//!   if n=assign_mu_glue then scan_glue(mu_val)@+else scan_glue(glue_val);
//!   trap_zero_glue;
//!   define(p,glue_ref,cur_val);
//!   end;
//!
//! @ When a glue register or parameter becomes zero, it will always point to
//! |zero_glue| because of the following procedure. (Exception: The tabskip
//! glue isn't trapped while preambles are being scanned.)
//!
//! @<Declare subprocedures for |prefixed_command|@>=
//! procedure trap_zero_glue;
//! begin if (width(cur_val)=0)and(stretch(cur_val)=0)and(shrink(cur_val)=0) then
//!   begin add_glue_ref(zero_glue);
//!   delete_glue_ref(cur_val); cur_val:=zero_glue;
//!   end;
//! end;
//!
