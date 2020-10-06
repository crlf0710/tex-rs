//! @* \[49] Mode-independent processing.
//! The long |main_control| procedure has now been fully specified, except for
//! certain activities that are independent of the current mode. These activities
//! do not change the current vlist or hlist or mlist; if they change anything,
//! it is the value of a parameter or the meaning of a control sequence.
//!
//! Assignments to values in |eqtb| can be global or local. Furthermore, a
//! control sequence can be defined to be `\.{\\long}' or `\.{\\outer}', and
//! it might or might not be expanded. The prefixes `\.{\\global}', `\.{\\long}',
//! and `\.{\\outer}' can occur in any order. Therefore we assign binary numeric
//! codes, making it possible to accumulate the union of all specified prefixes
//! by adding the corresponding codes.  (\PASCAL's |set| operations could also
//! have been used.)
//!
//! @<Put each...@>=
//! primitive("long",prefix,1);
//! @!@:long_}{\.{\\long} primitive@>
//! primitive("outer",prefix,2);
//! @!@:outer_}{\.{\\outer} primitive@>
//! primitive("global",prefix,4);
//! @!@:global_}{\.{\\global} primitive@>
//! primitive("def",def,0);
//! @!@:def_}{\.{\\def} primitive@>
//! primitive("gdef",def,1);
//! @!@:gdef_}{\.{\\gdef} primitive@>
//! primitive("edef",def,2);
//! @!@:edef_}{\.{\\edef} primitive@>
//! primitive("xdef",def,3);
//! @!@:xdef_}{\.{\\xdef} primitive@>
//!
//! @ @<Cases of |print_cmd_chr|...@>=
//! prefix: if chr_code=1 then print_esc("long")
//!   else if chr_code=2 then print_esc("outer")
//!   else print_esc("global");
//! def: if chr_code=0 then print_esc("def")
//!   else if chr_code=1 then print_esc("gdef")
//!   else if chr_code=2 then print_esc("edef")
//!   else print_esc("xdef");
//!
//! @ Every prefix, and every command code that might or might not be prefixed,
//! calls the action procedure |prefixed_command|. This routine accumulates
//! a sequence of prefixes until coming to a non-prefix, then it carries out
//! the command.
//!
//! @<Cases of |main_control| that don't...@>=
//! any_mode(toks_register),
//! any_mode(assign_toks),
//! any_mode(assign_int),
//! any_mode(assign_dimen),
//! any_mode(assign_glue),
//! any_mode(assign_mu_glue),
//! any_mode(assign_font_dimen),
//! any_mode(assign_font_int),
//! any_mode(set_aux),
//! any_mode(set_prev_graf),
//! any_mode(set_page_dimen),
//! any_mode(set_page_int),
//! any_mode(set_box_dimen),
//! any_mode(set_shape),
//! any_mode(def_code),
//! any_mode(def_family),
//! any_mode(set_font),
//! any_mode(def_font),
//! any_mode(register),
//! any_mode(advance),
//! any_mode(multiply),
//! any_mode(divide),
//! any_mode(prefix),
//! any_mode(let),
//! any_mode(shorthand_def),
//! any_mode(read_to_cs),
//! any_mode(def),
//! any_mode(set_box),
//! any_mode(hyph_data),
//! any_mode(set_interaction):prefixed_command;
//!
//! @ If the user says, e.g., `\.{\\global\\global}', the redundancy is
//! silently accepted.
//!
//! @<Declare act...@>=
//! @t\4@>@<Declare subprocedures for |prefixed_command|@>@t@>@;@/
//! procedure prefixed_command;
//! label done,exit;
//! var a:small_number; {accumulated prefix codes so far}
//! @!f:internal_font_number; {identifies a font}
//! @!j:halfword; {index into a \.{\\parshape} specification}
//! @!k:font_index; {index into |font_info|}
//! @!p,@!q:pointer; {for temporary short-term use}
//! @!n:integer; {ditto}
//! @!e:boolean; {should a definition be expanded? or was \.{\\let} not done?}
//! begin a:=0;
//! while cur_cmd=prefix do
//!   begin if not odd(a div cur_chr) then a:=a+cur_chr;
//!   @<Get the next non-blank non-relax...@>;
//!   if cur_cmd<=max_non_prefixed_command then
//!     @<Discard erroneous prefixes and |return|@>;
//!   end;
//! @<Discard the prefixes \.{\\long} and \.{\\outer} if they are irrelevant@>;
//! @<Adjust \(f)for the setting of \.{\\globaldefs}@>;
//! case cur_cmd of
//! @t\4@>@<Assignments@>@;
//! othercases confusion("prefix")
//! @:this can't happen prefix}{\quad prefix@>
//! endcases;
//! done: @<Insert a token saved by \.{\\afterassignment}, if any@>;
//! exit:end;
//!
//! @ @<Discard erroneous...@>=
//! begin print_err("You can't use a prefix with `");
//! @.You can't use a prefix with x@>
//! print_cmd_chr(cur_cmd,cur_chr); print_char("'");
//! help1("I'll pretend you didn't say \long or \outer or \global.");
//! back_error; return;
//! end
//!
//! @ @<Discard the prefixes...@>=
//! if (cur_cmd<>def)and(a mod 4<>0) then
//!   begin print_err("You can't use `"); print_esc("long"); print("' or `");
//!   print_esc("outer"); print("' with `");
//! @.You can't use \\long...@>
//!   print_cmd_chr(cur_cmd,cur_chr); print_char("'");
//!   help1("I'll pretend you didn't say \long or \outer here.");
//!   error;
//!   end
//!
//! @ The previous routine does not have to adjust |a| so that |a mod 4=0|,
//! since the following routines test for the \.{\\global} prefix as follows.
//!
//! @d global==(a>=4)
//! @d define(#)==if global then geq_define(#)@+else eq_define(#)
//! @d word_define(#)==if global then geq_word_define(#)@+else eq_word_define(#)
//!
//! @<Adjust \(f)for the setting of \.{\\globaldefs}@>=
//! if global_defs<>0 then
//!   if global_defs<0 then
//!     begin if global then a:=a-4;
//!     end
//!   else  begin if not global then a:=a+4;
//!     end
//!
//! @ When a control sequence is to be defined, by \.{\\def} or \.{\\let} or
//! something similar, the |get_r_token| routine will substitute a special
//! control sequence for a token that is not redefinable.
//!
//! @<Declare subprocedures for |prefixed_command|@>=
//! procedure get_r_token;
//! label restart;
//! begin restart: repeat get_token;
//! until cur_tok<>space_token;
//! if (cur_cs=0)or(cur_cs>frozen_control_sequence) then
//!   begin print_err("Missing control sequence inserted");
//! @.Missing control...@>
//!   help5("Please don't say `\def cs{...}', say `\def\cs{...}'.")@/
//!   ("I've inserted an inaccessible control sequence so that your")@/
//!   ("definition will be completed without mixing me up too badly.")@/
//!   ("You can recover graciously from this error, if you're")@/
//!   ("careful; see exercise 27.2 in The TeXbook.");
//! @:TeXbook}{\sl The \TeX book@>
//!   if cur_cs=0 then back_input;
//!   cur_tok:=cs_token_flag+frozen_protection; ins_error; goto restart;
//!   end;
//! end;
//!
//! @ @<Initialize table entries...@>=
//! text(frozen_protection):="inaccessible";
//!
//! @ Here's an example of the way many of the following routines operate.
//! (Unfortunately, they aren't all as simple as this.)
//!
//! @<Assignments@>=
//! set_font: define(cur_font_loc,data,cur_chr);
//!
//! @ When a |def| command has been scanned,
//! |cur_chr| is odd if the definition is supposed to be global, and
//! |cur_chr>=2| if the definition is supposed to be expanded.
//!
//! @<Assignments@>=
//! def: begin if odd(cur_chr)and not global and(global_defs>=0) then a:=a+4;
//!   e:=(cur_chr>=2); get_r_token; p:=cur_cs;
//!   q:=scan_toks(true,e); define(p,call+(a mod 4),def_ref);
//!   end;
//!
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
//! @ The various character code tables are changed by the |def_code| commands,
//! and the font families are declared by |def_family|.
//!
//! @<Put each...@>=
//! primitive("catcode",def_code,cat_code_base);
//! @!@:cat_code_}{\.{\\catcode} primitive@>
//! primitive("mathcode",def_code,math_code_base);
//! @!@:math_code_}{\.{\\mathcode} primitive@>
//! primitive("lccode",def_code,lc_code_base);
//! @!@:lc_code_}{\.{\\lccode} primitive@>
//! primitive("uccode",def_code,uc_code_base);
//! @!@:uc_code_}{\.{\\uccode} primitive@>
//! primitive("sfcode",def_code,sf_code_base);
//! @!@:sf_code_}{\.{\\sfcode} primitive@>
//! primitive("delcode",def_code,del_code_base);
//! @!@:del_code_}{\.{\\delcode} primitive@>
//! primitive("textfont",def_family,math_font_base);
//! @!@:text_font_}{\.{\\textfont} primitive@>
//! primitive("scriptfont",def_family,math_font_base+script_size);
//! @!@:script_font_}{\.{\\scriptfont} primitive@>
//! primitive("scriptscriptfont",def_family,math_font_base+script_script_size);
//! @!@:script_script_font_}{\.{\\scriptscriptfont} primitive@>
//!
//! @ @<Cases of |print_cmd_chr|...@>=
//! def_code: if chr_code=cat_code_base then print_esc("catcode")
//!   else if chr_code=math_code_base then print_esc("mathcode")
//!   else if chr_code=lc_code_base then print_esc("lccode")
//!   else if chr_code=uc_code_base then print_esc("uccode")
//!   else if chr_code=sf_code_base then print_esc("sfcode")
//!   else print_esc("delcode");
//! def_family: print_size(chr_code-math_font_base);
//!
//! @ The different types of code values have different legal ranges; the
//! following program is careful to check each case properly.
//!
//! @<Assignments@>=
//! def_code: begin @<Let |n| be the largest legal code value, based on |cur_chr|@>;
//!   p:=cur_chr; scan_char_num; p:=p+cur_val; scan_optional_equals;
//!   scan_int;
//!   if ((cur_val<0)and(p<del_code_base))or(cur_val>n) then
//!     begin print_err("Invalid code ("); print_int(cur_val);
//! @.Invalid code@>
//!     if p<del_code_base then print("), should be in the range 0..")
//!     else print("), should be at most ");
//!     print_int(n);
//!     help1("I'm going to use 0 instead of that illegal code value.");@/
//!     error; cur_val:=0;
//!     end;
//!   if p<math_code_base then define(p,data,cur_val)
//!   else if p<del_code_base then define(p,data,hi(cur_val))
//!   else word_define(p,cur_val);
//!   end;
//!
//! @ @<Let |n| be the largest...@>=
//! if cur_chr=cat_code_base then n:=max_char_code
//! else if cur_chr=math_code_base then n:=@'100000
//! else if cur_chr=sf_code_base then n:=@'77777
//! else if cur_chr=del_code_base then n:=@'77777777
//! else n:=255
//!
//! @ @<Assignments@>=
//! def_family: begin p:=cur_chr; scan_four_bit_int; p:=p+cur_val;
//!   scan_optional_equals; scan_font_ident; define(p,data,cur_val);
//!   end;
//!
//! @ Next we consider changes to \TeX's numeric registers.
//!
//! @<Assignments@>=
//! register,advance,multiply,divide: do_register_command(a);
//!
//! @ We use the fact that |register<advance<multiply<divide|.
//!
//! @<Declare subprocedures for |prefixed_command|@>=
//! procedure do_register_command(@!a:small_number);
//! label found,exit;
//! var l,@!q,@!r,@!s:pointer; {for list manipulation}
//! @!p:int_val..mu_val; {type of register involved}
//! begin q:=cur_cmd;
//! @<Compute the register location |l| and its type |p|; but |return| if invalid@>;
//! if q=register then scan_optional_equals
//! else if scan_keyword("by") then do_nothing; {optional `\.{by}'}
//! @.by@>
//! arith_error:=false;
//! if q<multiply then @<Compute result of |register| or
//!     |advance|, put it in |cur_val|@>
//! else @<Compute result of |multiply| or |divide|, put it in |cur_val|@>;
//! if arith_error then
//!   begin print_err("Arithmetic overflow");
//! @.Arithmetic overflow@>
//!   help2("I can't carry out that multiplication or division,")@/
//!     ("since the result is out of range.");
//!   if p>=glue_val then delete_glue_ref(cur_val);
//!   error; return;
//!   end;
//! if p<glue_val then word_define(l,cur_val)
//! else  begin trap_zero_glue; define(l,glue_ref,cur_val);
//!   end;
//! exit: end;
//!
//! @ Here we use the fact that the consecutive codes |int_val..mu_val| and
//! |assign_int..assign_mu_glue| correspond to each other nicely.
//!
//! @<Compute the register location |l| and its type |p|...@>=
//! begin if q<>register then
//!   begin get_x_token;
//!   if (cur_cmd>=assign_int)and(cur_cmd<=assign_mu_glue) then
//!     begin l:=cur_chr; p:=cur_cmd-assign_int; goto found;
//!     end;
//!   if cur_cmd<>register then
//!     begin print_err("You can't use `"); print_cmd_chr(cur_cmd,cur_chr);
//! @.You can't use x after ...@>
//!     print("' after "); print_cmd_chr(q,0);
//!     help1("I'm forgetting what you said and not changing anything.");
//!     error; return;
//!     end;
//!   end;
//! p:=cur_chr; scan_eight_bit_int;
//! case p of
//! int_val: l:=cur_val+count_base;
//! dimen_val: l:=cur_val+scaled_base;
//! glue_val: l:=cur_val+skip_base;
//! mu_val: l:=cur_val+mu_skip_base;
//! end; {there are no other cases}
//! end;
//! found:
//!
//! @ @<Compute result of |register| or |advance|...@>=
//! if p<glue_val then
//!   begin if p=int_val then scan_int@+else scan_normal_dimen;
//!   if q=advance then cur_val:=cur_val+eqtb[l].int;
//!   end
//! else  begin scan_glue(p);
//!   if q=advance then @<Compute the sum of two glue specs@>;
//!   end
//!
//! @ @<Compute the sum of two glue specs@>=
//! begin q:=new_spec(cur_val); r:=equiv(l);
//! delete_glue_ref(cur_val);
//! width(q):=width(q)+width(r);
//! if stretch(q)=0 then stretch_order(q):=normal;
//! if stretch_order(q)=stretch_order(r) then stretch(q):=stretch(q)+stretch(r)
//! else if (stretch_order(q)<stretch_order(r))and(stretch(r)<>0) then
//!   begin stretch(q):=stretch(r); stretch_order(q):=stretch_order(r);
//!   end;
//! if shrink(q)=0 then shrink_order(q):=normal;
//! if shrink_order(q)=shrink_order(r) then shrink(q):=shrink(q)+shrink(r)
//! else if (shrink_order(q)<shrink_order(r))and(shrink(r)<>0) then
//!   begin shrink(q):=shrink(r); shrink_order(q):=shrink_order(r);
//!   end;
//! cur_val:=q;
//! end
//!
//! @ @<Compute result of |multiply| or |divide|...@>=
//! begin scan_int;
//! if p<glue_val then
//!   if q=multiply then
//!     if p=int_val then cur_val:=mult_integers(eqtb[l].int,cur_val)
//!     else cur_val:=nx_plus_y(eqtb[l].int,cur_val,0)
//!   else cur_val:=x_over_n(eqtb[l].int,cur_val)
//! else  begin s:=equiv(l); r:=new_spec(s);
//!   if q=multiply then
//!     begin width(r):=nx_plus_y(width(s),cur_val,0);
//!     stretch(r):=nx_plus_y(stretch(s),cur_val,0);
//!     shrink(r):=nx_plus_y(shrink(s),cur_val,0);
//!     end
//!   else  begin width(r):=x_over_n(width(s),cur_val);
//!     stretch(r):=x_over_n(stretch(s),cur_val);
//!     shrink(r):=x_over_n(shrink(s),cur_val);
//!     end;
//!   cur_val:=r;
//!   end;
//! end
//!
//! @ The processing of boxes is somewhat different, because we may need
//! to scan and create an entire box before we actually change the value of the old
//! one.
//!
//! @<Assignments@>=
//! set_box: begin scan_eight_bit_int;
//!   if global then n:=256+cur_val@+else n:=cur_val;
//!   scan_optional_equals;
//!   if set_box_allowed then scan_box(box_flag+n)
//!   else begin print_err("Improper "); print_esc("setbox");
//! @.Improper \\setbox@>
//!     help2("Sorry, \setbox is not allowed after \halign in a display,")@/
//!     ("or between \accent and an accented character."); error;
//!     end;
//!   end;
//!
//! @ The |space_factor| or |prev_depth| settings are changed when a |set_aux|
//! command is sensed. Similarly, |prev_graf| is changed in the presence of
//! |set_prev_graf|, and |dead_cycles| or |insert_penalties| in the presence of
//! |set_page_int|. These definitions are always global.
//!
//! When some dimension of a box register is changed, the change isn't exactly
//! global; but \TeX\ does not look at the \.{\\global} switch.
//!
//! @<Assignments@>=
//! set_aux:alter_aux;
//! set_prev_graf:alter_prev_graf;
//! set_page_dimen:alter_page_so_far;
//! set_page_int:alter_integer;
//! set_box_dimen:alter_box_dimen;
//!
//! @ @<Declare subprocedures for |prefixed_command|@>=
//! procedure alter_aux;
//! var c:halfword; {|hmode| or |vmode|}
//! begin if cur_chr<>abs(mode) then report_illegal_case
//! else  begin c:=cur_chr; scan_optional_equals;
//!   if c=vmode then
//!     begin scan_normal_dimen; prev_depth:=cur_val;
//!     end
//!   else  begin scan_int;
//!     if (cur_val<=0)or(cur_val>32767) then
//!       begin print_err("Bad space factor");
//! @.Bad space factor@>
//!       help1("I allow only values in the range 1..32767 here.");
//!       int_error(cur_val);
//!       end
//!     else space_factor:=cur_val;
//!     end;
//!   end;
//! end;
//!
//! @ @<Declare subprocedures for |prefixed_command|@>=
//! procedure alter_prev_graf;
//! var p:0..nest_size; {index into |nest|}
//! begin nest[nest_ptr]:=cur_list; p:=nest_ptr;
//! while abs(nest[p].mode_field)<>vmode do decr(p);
//! scan_optional_equals; scan_int;
//! if cur_val<0 then
//!   begin print_err("Bad "); print_esc("prevgraf");
//! @.Bad \\prevgraf@>
//!   help1("I allow only nonnegative values here.");
//!   int_error(cur_val);
//!   end
//! else  begin nest[p].pg_field:=cur_val; cur_list:=nest[nest_ptr];
//!   end;
//! end;
//!
//! @ @<Declare subprocedures for |prefixed_command|@>=
//! procedure alter_page_so_far;
//! var c:0..7; {index into |page_so_far|}
//! begin c:=cur_chr; scan_optional_equals; scan_normal_dimen;
//! page_so_far[c]:=cur_val;
//! end;
//!
//! @ @<Declare subprocedures for |prefixed_command|@>=
//! procedure alter_integer;
//! var c:0..1; {0 for \.{\\deadcycles}, 1 for \.{\\insertpenalties}}
//! begin c:=cur_chr; scan_optional_equals; scan_int;
//! if c=0 then dead_cycles:=cur_val
//! else insert_penalties:=cur_val;
//! end;
//!
//! @ @<Declare subprocedures for |prefixed_command|@>=
//! procedure alter_box_dimen;
//! var c:small_number; {|width_offset| or |height_offset| or |depth_offset|}
//! @!b:eight_bits; {box number}
//! begin c:=cur_chr; scan_eight_bit_int; b:=cur_val; scan_optional_equals;
//! scan_normal_dimen;
//! if box(b)<>null then mem[box(b)+c].sc:=cur_val;
//! end;
//!
//! @ Paragraph shapes are set up in the obvious way.
//!
//! @<Assignments@>=
//! set_shape: begin scan_optional_equals; scan_int; n:=cur_val;
//!   if n<=0 then p:=null
//!   else  begin p:=get_node(2*n+1); info(p):=n;
//!     for j:=1 to n do
//!       begin scan_normal_dimen;
//!       mem[p+2*j-1].sc:=cur_val; {indentation}
//!       scan_normal_dimen;
//!       mem[p+2*j].sc:=cur_val; {width}
//!       end;
//!     end;
//!   define(par_shape_loc,shape_ref,p);
//!   end;
//!
//! @ Here's something that isn't quite so obvious. It guarantees that
//! |info(par_shape_ptr)| can hold any positive~|n| for which |get_node(2*n+1)|
//! doesn't overflow the memory capacity.
//!
//! @<Check the ``constant''...@>=
//! if 2*max_halfword<mem_top-mem_min then bad:=41;
//!
//! @ New hyphenation data is loaded by the |hyph_data| command.
//!
//! @<Put each...@>=
//! primitive("hyphenation",hyph_data,0);
//! @!@:hyphenation_}{\.{\\hyphenation} primitive@>
//! primitive("patterns",hyph_data,1);
//! @!@:patterns_}{\.{\\patterns} primitive@>
//!
//! @ @<Cases of |print_cmd_chr|...@>=
//! hyph_data: if chr_code=1 then print_esc("patterns")
//!   else print_esc("hyphenation");
//!
//! @ @<Assignments@>=
//! hyph_data: if cur_chr=1 then
//!     begin @!init new_patterns; goto done;@;@+tini@/
//!     print_err("Patterns can be loaded only by INITEX");
//! @.Patterns can be...@>
//!     help0; error;
//!     repeat get_token; until cur_cmd=right_brace; {flush the patterns}
//!     return;
//!     end
//!   else  begin new_hyph_exceptions; goto done;
//!     end;
//!
//! @ All of \TeX's parameters are kept in |eqtb| except the font information,
//! the interaction mode, and the hyphenation tables; these are strictly global.
//!
//! @<Assignments@>=
//! assign_font_dimen: begin find_font_dimen(true); k:=cur_val;
//!   scan_optional_equals; scan_normal_dimen; font_info[k].sc:=cur_val;
//!   end;
//! assign_font_int: begin n:=cur_chr; scan_font_ident; f:=cur_val;
//!   scan_optional_equals; scan_int;
//!   if n=0 then hyphen_char[f]:=cur_val@+else skew_char[f]:=cur_val;
//!   end;
//!
//! @ @<Put each...@>=
//! primitive("hyphenchar",assign_font_int,0);
//! @!@:hyphen_char_}{\.{\\hyphenchar} primitive@>
//! primitive("skewchar",assign_font_int,1);
//! @!@:skew_char_}{\.{\\skewchar} primitive@>
//!
//! @ @<Cases of |print_cmd_chr|...@>=
//! assign_font_int: if chr_code=0 then print_esc("hyphenchar")
//!   else print_esc("skewchar");
//!
//! @ Here is where the information for a new font gets loaded.
//!
//! @<Assignments@>=
//! def_font: new_font(a);
//!
//! @ @<Declare subprocedures for |prefixed_command|@>=
//! procedure new_font(@!a:small_number);
//! label common_ending;
//! var u:pointer; {user's font identifier}
//! @!s:scaled; {stated ``at'' size, or negative of scaled magnification}
//! @!f:internal_font_number; {runs through existing fonts}
//! @!t:str_number; {name for the frozen font identifier}
//! @!old_setting:0..max_selector; {holds |selector| setting}
//! @!flushable_string:str_number; {string not yet referenced}
//! begin if job_name=0 then open_log_file;
//!   {avoid confusing \.{texput} with the font name}
//! @.texput@>
//! get_r_token; u:=cur_cs;
//! if u>=hash_base then t:=text(u)
//! else if u>=single_base then
//!   if u=null_cs then t:="FONT"@+else t:=u-single_base
//! else  begin old_setting:=selector; selector:=new_string;
//!   print("FONT"); print(u-active_base); selector:=old_setting;
//! @.FONTx@>
//!   str_room(1); t:=make_string;
//!   end;
//! define(u,set_font,null_font); scan_optional_equals; scan_file_name;
//! @<Scan the font size specification@>;
//! @<If this font has already been loaded, set |f| to the internal
//!   font number and |goto common_ending|@>;
//! f:=read_font_info(u,cur_name,cur_area,s);
//! common_ending: equiv(u):=f; eqtb[font_id_base+f]:=eqtb[u]; font_id_text(f):=t;
//! end;
//!
//! @ @<Scan the font size specification@>=
//! name_in_progress:=true; {this keeps |cur_name| from being changed}
//! if scan_keyword("at") then @<Put the \(p)(positive) `at' size into |s|@>
//! @.at@>
//! else if scan_keyword("scaled") then
//! @.scaled@>
//!   begin scan_int; s:=-cur_val;
//!   if (cur_val<=0)or(cur_val>32768) then
//!     begin print_err("Illegal magnification has been changed to 1000");@/
//! @.Illegal magnification...@>
//!     help1("The magnification ratio must be between 1 and 32768.");
//!     int_error(cur_val); s:=-1000;
//!     end;
//!   end
//! else s:=-1000;
//! name_in_progress:=false
//!
//! @ @<Put the \(p)(positive) `at' size into |s|@>=
//! begin scan_normal_dimen; s:=cur_val;
//! if (s<=0)or(s>=@'1000000000) then
//!   begin print_err("Improper `at' size (");
//!   print_scaled(s); print("pt), replaced by 10pt");
//! @.Improper `at' size...@>
//!   help2("I can only handle fonts at positive sizes that are")@/
//!   ("less than 2048pt, so I've changed what you said to 10pt.");
//!   error; s:=10*unity;
//!   end;
//! end
//!
//! @ When the user gives a new identifier to a font that was previously loaded,
//! the new name becomes the font identifier of record. Font names `\.{xyz}' and
//! `\.{XYZ}' are considered to be different.
//!
//! @<If this font has already been loaded...@>=
//! flushable_string:=str_ptr-1;
//! for f:=font_base+1 to font_ptr do
//!   if str_eq_str(font_name[f],cur_name)and str_eq_str(font_area[f],cur_area) then
//!     begin if cur_name=flushable_string then
//!       begin flush_string; cur_name:=font_name[f];
//!       end;
//!     if s>0 then
//!       begin if s=font_size[f] then goto common_ending;
//!       end
//!     else if font_size[f]=xn_over_d(font_dsize[f],-s,1000) then
//!       goto common_ending;
//!     end
//!
//! @ @<Cases of |print_cmd_chr|...@>=
//! set_font:begin print("select font "); slow_print(font_name[chr_code]);
//!   if font_size[chr_code]<>font_dsize[chr_code] then
//!     begin print(" at "); print_scaled(font_size[chr_code]);
//!     print("pt");
//!     end;
//!   end;
//!
//! @ @<Put each...@>=
//! primitive("batchmode",set_interaction,batch_mode);
//! @!@:batch_mode_}{\.{\\batchmode} primitive@>
//! primitive("nonstopmode",set_interaction,nonstop_mode);
//! @!@:nonstop_mode_}{\.{\\nonstopmode} primitive@>
//! primitive("scrollmode",set_interaction,scroll_mode);
//! @!@:scroll_mode_}{\.{\\scrollmode} primitive@>
//! primitive("errorstopmode",set_interaction,error_stop_mode);
//! @!@:error_stop_mode_}{\.{\\errorstopmode} primitive@>
//!
//! @ @<Cases of |print_cmd_chr|...@>=
//! set_interaction: case chr_code of
//!   batch_mode: print_esc("batchmode");
//!   nonstop_mode: print_esc("nonstopmode");
//!   scroll_mode: print_esc("scrollmode");
//!   othercases print_esc("errorstopmode")
//!   endcases;
//!
//! @ @<Assignments@>=
//! set_interaction: new_interaction;
//!
//! @ @<Declare subprocedures for |prefixed_command|@>=
//! procedure new_interaction;
//! begin print_ln;
//! interaction:=cur_chr;
//! @<Initialize the print |selector| based on |interaction|@>;
//! if log_opened then selector:=selector+2;
//! end;
//!
//! @ The \.{\\afterassignment} command puts a token into the global
//! variable |after_token|. This global variable is examined just after
//! every assignment has been performed.
//!
//! @<Glob...@>=
//! @!after_token:halfword; {zero, or a saved token}
//!
//! @ @<Set init...@>=
//! after_token:=0;
//!
//! @ @<Cases of |main_control| that don't...@>=
//! any_mode(after_assignment):begin get_token; after_token:=cur_tok;
//!   end;
//!
//! @ @<Insert a token saved by \.{\\afterassignment}, if any@>=
//! if after_token<>0 then
//!   begin cur_tok:=after_token; back_input; after_token:=0;
//!   end
//!
//! @ Here is a procedure that might be called `Get the next non-blank non-relax
//! non-call non-assignment token'.
//!
//! @<Declare act...@>=
//! procedure do_assignments;
//! label exit;
//! begin loop begin @<Get the next non-blank non-relax...@>;
//!   if cur_cmd<=max_non_prefixed_command then return;
//!   set_box_allowed:=false; prefixed_command; set_box_allowed:=true;
//!   end;
//! exit:end;
//!
//! @ @<Cases of |main_control| that don't...@>=
//! any_mode(after_group):begin get_token; save_for_after(cur_tok);
//!   end;
//!
//! @ Files for \.{\\read} are opened and closed by the |in_stream| command.
//!
//! @<Put each...@>=
//! primitive("openin",in_stream,1);
//! @!@:open_in_}{\.{\\openin} primitive@>
//! primitive("closein",in_stream,0);
//! @!@:close_in_}{\.{\\closein} primitive@>
//!
//! @ @<Cases of |print_cmd_chr|...@>=
//! in_stream: if chr_code=0 then print_esc("closein")
//!   else print_esc("openin");
//!
//! @ @<Cases of |main_control| that don't...@>=
//! any_mode(in_stream): open_or_close_in;
//!
//! @ @<Declare act...@>=
//! procedure open_or_close_in;
//! var c:0..1; {1 for \.{\\openin}, 0 for \.{\\closein}}
//! @!n:0..15; {stream number}
//! begin c:=cur_chr; scan_four_bit_int; n:=cur_val;
//! if read_open[n]<>closed then
//!   begin a_close(read_file[n]); read_open[n]:=closed;
//!   end;
//! if c<>0 then
//!   begin scan_optional_equals; scan_file_name;
//!   if cur_ext="" then cur_ext:=".tex";
//!   pack_cur_name;
//!   if a_open_in(read_file[n]) then read_open[n]:=just_open;
//!   end;
//! end;
//!
//! @ The user can issue messages to the terminal, regardless of the
//! current mode.
//!
//! @<Cases of |main_control| that don't...@>=
//! any_mode(message):issue_message;
//!
//! @ @<Put each...@>=
//! primitive("message",message,0);
//! @!@:message_}{\.{\\message} primitive@>
//! primitive("errmessage",message,1);
//! @!@:err_message_}{\.{\\errmessage} primitive@>
//!
//! @ @<Cases of |print_cmd_chr|...@>=
//! message: if chr_code=0 then print_esc("message")
//!   else print_esc("errmessage");
//!
//! @ @<Declare act...@>=
//! procedure issue_message;
//! var old_setting:0..max_selector; {holds |selector| setting}
//! @!c:0..1; {identifies \.{\\message} and \.{\\errmessage}}
//! @!s:str_number; {the message}
//! begin c:=cur_chr; link(garbage):=scan_toks(false,true);
//! old_setting:=selector; selector:=new_string;
//! token_show(def_ref); selector:=old_setting;
//! flush_list(def_ref);
//! str_room(1); s:=make_string;
//! if c=0 then @<Print string |s| on the terminal@>
//! else @<Print string |s| as an error message@>;
//! flush_string;
//! end;
//!
//! @ @<Print string |s| on the terminal@>=
//! begin if term_offset+length(s)>max_print_line-2 then print_ln
//! else if (term_offset>0)or(file_offset>0) then print_char(" ");
//! slow_print(s); update_terminal;
//! end
//!
//! @ If \.{\\errmessage} occurs often in |scroll_mode|, without user-defined
//! \.{\\errhelp}, we don't want to give a long help message each time. So we
//! give a verbose explanation only once.
//!
//! @<Glob...@>=
//! @!long_help_seen:boolean; {has the long \.{\\errmessage} help been used?}
//!
//! @ @<Set init...@>=long_help_seen:=false;
//!
//! @ @<Print string |s| as an error message@>=
//! begin print_err(""); slow_print(s);
//! if err_help<>null then use_err_help:=true
//! else if long_help_seen then help1("(That was another \errmessage.)")
//! else  begin if interaction<error_stop_mode then long_help_seen:=true;
//!   help4("This error message was generated by an \errmessage")@/
//!   ("command, so I can't give any explicit help.")@/
//!   ("Pretend that you're Hercule Poirot: Examine all clues,")@/
//! @^Poirot, Hercule@>
//!   ("and deduce the truth by order and method.");
//!   end;
//! error; use_err_help:=false;
//! end
//!
//! @ The |error| routine calls on |give_err_help| if help is requested from
//! the |err_help| parameter.
//!
//! @p procedure give_err_help;
//! begin token_show(err_help);
//! end;
//!
//! @ The \.{\\uppercase} and \.{\\lowercase} commands are implemented by
//! building a token list and then changing the cases of the letters in it.
//!
//! @<Cases of |main_control| that don't...@>=
//! any_mode(case_shift):shift_case;
//!
//! @ @<Put each...@>=
//! primitive("lowercase",case_shift,lc_code_base);
//! @!@:lowercase_}{\.{\\lowercase} primitive@>
//! primitive("uppercase",case_shift,uc_code_base);
//! @!@:uppercase_}{\.{\\uppercase} primitive@>
//!
//! @ @<Cases of |print_cmd_chr|...@>=
//! case_shift:if chr_code=lc_code_base then print_esc("lowercase")
//!   else print_esc("uppercase");
//!
//! @ @<Declare act...@>=
//! procedure shift_case;
//! var b:pointer; {|lc_code_base| or |uc_code_base|}
//! @!p:pointer; {runs through the token list}
//! @!t:halfword; {token}
//! @!c:eight_bits; {character code}
//! begin b:=cur_chr; p:=scan_toks(false,false); p:=link(def_ref);
//! while p<>null do
//!   begin @<Change the case of the token in |p|, if a change is appropriate@>;
//!   p:=link(p);
//!   end;
//! back_list(link(def_ref)); free_avail(def_ref); {omit reference count}
//! end;
//!
//! @ When the case of a |chr_code| changes, we don't change the |cmd|.
//! We also change active characters, using the fact that
//! |cs_token_flag+active_base| is a multiple of~256.
//! @^data structure assumptions@>
//!
//! @<Change the case of the token in |p|, if a change is appropriate@>=
//! t:=info(p);
//! if t<cs_token_flag+single_base then
//!   begin c:=t mod 256;
//!   if equiv(b+c)<>0 then info(p):=t-c+equiv(b+c);
//!   end
//!
//! @ We come finally to the last pieces missing from |main_control|, namely the
//! `\.{\\show}' commands that are useful when debugging.
//!
//! @<Cases of |main_control| that don't...@>=
//! any_mode(xray): show_whatever;
//!
//! @ @d show_code=0 { \.{\\show} }
//! @d show_box_code=1 { \.{\\showbox} }
//! @d show_the_code=2 { \.{\\showthe} }
//! @d show_lists=3 { \.{\\showlists} }
//!
//! @<Put each...@>=
//! primitive("show",xray,show_code);
//! @!@:show_}{\.{\\show} primitive@>
//! primitive("showbox",xray,show_box_code);
//! @!@:show_box_}{\.{\\showbox} primitive@>
//! primitive("showthe",xray,show_the_code);
//! @!@:show_the_}{\.{\\showthe} primitive@>
//! primitive("showlists",xray,show_lists);
//! @!@:show_lists_}{\.{\\showlists} primitive@>
//!
//! @ @<Cases of |print_cmd_chr|...@>=
//! xray: case chr_code of
//!   show_box_code:print_esc("showbox");
//!   show_the_code:print_esc("showthe");
//!   show_lists:print_esc("showlists");
//!   othercases print_esc("show")
//!   endcases;
//!
//! @ @<Declare act...@>=
//! procedure show_whatever;
//! label common_ending;
//! var p:pointer; {tail of a token list to show}
//! begin case cur_chr of
//! show_lists: begin begin_diagnostic; show_activities;
//!   end;
//! show_box_code: @<Show the current contents of a box@>;
//! show_code: @<Show the current meaning of a token, then |goto common_ending|@>;
//! othercases @<Show the current value of some parameter or register,
//!   then |goto common_ending|@>
//! endcases;@/
//! @<Complete a potentially long \.{\\show} command@>;
//! common_ending: if interaction<error_stop_mode then
//!   begin help0; decr(error_count);
//!   end
//! else if tracing_online>0 then
//!   begin@t@>@;@/
//!   help3("This isn't an error message; I'm just \showing something.")@/
//!   ("Type `I\show...' to show more (e.g., \show\cs,")@/
//!   ("\showthe\count10, \showbox255, \showlists).");
//!   end
//! else  begin@t@>@;@/
//!   help5("This isn't an error message; I'm just \showing something.")@/
//!   ("Type `I\show...' to show more (e.g., \show\cs,")@/
//!   ("\showthe\count10, \showbox255, \showlists).")@/
//!   ("And type `I\tracingonline=1\show...' to show boxes and")@/
//!   ("lists on your terminal as well as in the transcript file.");
//!   end;
//! error;
//! end;
//!
//! @ @<Show the current meaning of a token...@>=
//! begin get_token;
//! if interaction=error_stop_mode then wake_up_terminal;
//! print_nl("> ");
//! if cur_cs<>0 then
//!   begin sprint_cs(cur_cs); print_char("=");
//!   end;
//! print_meaning; goto common_ending;
//! end
//!
//! @ @<Cases of |print_cmd_chr|...@>=
//! undefined_cs: print("undefined");
//! call: print("macro");
//! long_call: print_esc("long macro");
//! outer_call: print_esc("outer macro");
//! long_outer_call: begin print_esc("long"); print_esc("outer macro");
//!   end;
//! end_template: print_esc("outer endtemplate");
//!
//! @ @<Show the current contents of a box@>=
//! begin scan_eight_bit_int; begin_diagnostic;
//! print_nl("> \box"); print_int(cur_val); print_char("=");
//! if box(cur_val)=null then print("void")
//! else show_box(box(cur_val));
//! end
//!
//! @ @<Show the current value of some parameter...@>=
//! begin p:=the_toks;
//! if interaction=error_stop_mode then wake_up_terminal;
//! print_nl("> "); token_show(temp_head);
//! flush_list(link(temp_head)); goto common_ending;
//! end
//!
//! @ @<Complete a potentially long \.{\\show} command@>=
//! end_diagnostic(true); print_err("OK");
//! @.OK@>
//! if selector=term_and_log then if tracing_online<=0 then
//!   begin selector:=term_only; print(" (see the transcript file)");
//!   selector:=term_and_log;
//!   end
//!
