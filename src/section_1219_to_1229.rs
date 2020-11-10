//! @ Both \.{\\let} and \.{\\futurelet} share the command code |let|.
//!
//! @<Put each...@>=
//! primitive("let",let,normal);@/
//! @!@:let_}{\.{\\let} primitive@>
//! primitive("futurelet",let,normal+1);@/
//! @!@:future_let_}{\.{\\futurelet} primitive@>
//!
//! @ @<Cases of |print_cmd_chr|...@>=
//! let: if chr_code<>normal then print_esc("futurelet")@+else print_esc("let");
//!
//! @ @<Assignments@>=
//! let:  begin n:=cur_chr;
//!   get_r_token; p:=cur_cs;
//!   if n=normal then
//!     begin repeat get_token;
//!     until cur_cmd<>spacer;
//!     if cur_tok=other_token+"=" then
//!       begin get_token;
//!       if cur_cmd=spacer then get_token;
//!       end;
//!     end
//!   else  begin get_token; q:=cur_tok; get_token; back_input;
//!     cur_tok:=q; back_input; {look ahead, then back up}
//!     end; {note that |back_input| doesn't affect |cur_cmd|, |cur_chr|}
//!   if cur_cmd>=call then add_token_ref(cur_chr);
//!   define(p,cur_cmd,cur_chr);
//!   end;
//!
//! @ A \.{\\chardef} creates a control sequence whose |cmd| is |char_given|;
//! a \.{\\mathchardef} creates a control sequence whose |cmd| is |math_given|;
//! and the corresponding |chr| is the character code or math code. A \.{\\countdef}
//! or \.{\\dimendef} or \.{\\skipdef} or \.{\\muskipdef} creates a control
//! sequence whose |cmd| is |assign_int| or \dots\ or |assign_mu_glue|, and the
//! corresponding |chr| is the |eqtb| location of the internal register in question.
//!
//! @d char_def_code=0 {|shorthand_def| for \.{\\chardef}}
//! @d math_char_def_code=1 {|shorthand_def| for \.{\\mathchardef}}
//! @d count_def_code=2 {|shorthand_def| for \.{\\countdef}}
//! @d dimen_def_code=3 {|shorthand_def| for \.{\\dimendef}}
//! @d skip_def_code=4 {|shorthand_def| for \.{\\skipdef}}
//! @d mu_skip_def_code=5 {|shorthand_def| for \.{\\muskipdef}}
//! @d toks_def_code=6 {|shorthand_def| for \.{\\toksdef}}
//!
//! @<Put each...@>=
//! primitive("chardef",shorthand_def,char_def_code);@/
//! @!@:char_def_}{\.{\\chardef} primitive@>
//! primitive("mathchardef",shorthand_def,math_char_def_code);@/
//! @!@:math_char_def_}{\.{\\mathchardef} primitive@>
//! primitive("countdef",shorthand_def,count_def_code);@/
//! @!@:count_def_}{\.{\\countdef} primitive@>
//! primitive("dimendef",shorthand_def,dimen_def_code);@/
//! @!@:dimen_def_}{\.{\\dimendef} primitive@>
//! primitive("skipdef",shorthand_def,skip_def_code);@/
//! @!@:skip_def_}{\.{\\skipdef} primitive@>
//! primitive("muskipdef",shorthand_def,mu_skip_def_code);@/
//! @!@:mu_skip_def_}{\.{\\muskipdef} primitive@>
//! primitive("toksdef",shorthand_def,toks_def_code);@/
//! @!@:toks_def_}{\.{\\toksdef} primitive@>
//!
//! @ @<Cases of |print_cmd_chr|...@>=
//! shorthand_def: case chr_code of
//!   char_def_code: print_esc("chardef");
//!   math_char_def_code: print_esc("mathchardef");
//!   count_def_code: print_esc("countdef");
//!   dimen_def_code: print_esc("dimendef");
//!   skip_def_code: print_esc("skipdef");
//!   mu_skip_def_code: print_esc("muskipdef");
//!   othercases print_esc("toksdef")
//!   endcases;
//! char_given: begin print_esc("char"); print_hex(chr_code);
//!   end;
//! math_given: begin print_esc("mathchar"); print_hex(chr_code);
//!   end;
//!
//! @ We temporarily define |p| to be |relax|, so that an occurrence of |p|
//! while scanning the definition will simply stop the scanning instead of
//! producing an ``undefined control sequence'' error or expanding the
//! previous meaning.  This allows, for instance, `\.{\\chardef\\foo=123\\foo}'.
//!
//! @<Assignments@>=
//! shorthand_def: begin n:=cur_chr; get_r_token; p:=cur_cs; define(p,relax,256);
//!   scan_optional_equals;
//!   case n of
//!   char_def_code: begin scan_char_num; define(p,char_given,cur_val);
//!     end;
//!   math_char_def_code: begin scan_fifteen_bit_int; define(p,math_given,cur_val);
//!     end;
//!   othercases begin scan_eight_bit_int;
//!     case n of
//!     count_def_code: define(p,assign_int,count_base+cur_val);
//!     dimen_def_code: define(p,assign_dimen,scaled_base+cur_val);
//!     skip_def_code: define(p,assign_glue,skip_base+cur_val);
//!     mu_skip_def_code: define(p,assign_mu_glue,mu_skip_base+cur_val);
//!     toks_def_code: define(p,assign_toks,toks_base+cur_val);
//!     end; {there are no other cases}
//!     end
//!   endcases;
//!   end;
//!
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
