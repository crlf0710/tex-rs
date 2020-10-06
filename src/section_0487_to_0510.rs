//! @* \[28] Conditional processing.
//! We consider now the way \TeX\ handles various kinds of \.{\\if} commands.
//!
//! @d if_char_code=0 { `\.{\\if}' }
//! @d if_cat_code=1 { `\.{\\ifcat}' }
//! @d if_int_code=2 { `\.{\\ifnum}' }
//! @d if_dim_code=3 { `\.{\\ifdim}' }
//! @d if_odd_code=4 { `\.{\\ifodd}' }
//! @d if_vmode_code=5 { `\.{\\ifvmode}' }
//! @d if_hmode_code=6 { `\.{\\ifhmode}' }
//! @d if_mmode_code=7 { `\.{\\ifmmode}' }
//! @d if_inner_code=8 { `\.{\\ifinner}' }
//! @d if_void_code=9 { `\.{\\ifvoid}' }
//! @d if_hbox_code=10 { `\.{\\ifhbox}' }
//! @d if_vbox_code=11 { `\.{\\ifvbox}' }
//! @d ifx_code=12 { `\.{\\ifx}' }
//! @d if_eof_code=13 { `\.{\\ifeof}' }
//! @d if_true_code=14 { `\.{\\iftrue}' }
//! @d if_false_code=15 { `\.{\\iffalse}' }
//! @d if_case_code=16 { `\.{\\ifcase}' }
//!
//! @<Put each...@>=
//! primitive("if",if_test,if_char_code);
//! @!@:if_char_}{\.{\\if} primitive@>
//! primitive("ifcat",if_test,if_cat_code);
//! @!@:if_cat_code_}{\.{\\ifcat} primitive@>
//! primitive("ifnum",if_test,if_int_code);
//! @!@:if_int_}{\.{\\ifnum} primitive@>
//! primitive("ifdim",if_test,if_dim_code);
//! @!@:if_dim_}{\.{\\ifdim} primitive@>
//! primitive("ifodd",if_test,if_odd_code);
//! @!@:if_odd_}{\.{\\ifodd} primitive@>
//! primitive("ifvmode",if_test,if_vmode_code);
//! @!@:if_vmode_}{\.{\\ifvmode} primitive@>
//! primitive("ifhmode",if_test,if_hmode_code);
//! @!@:if_hmode_}{\.{\\ifhmode} primitive@>
//! primitive("ifmmode",if_test,if_mmode_code);
//! @!@:if_mmode_}{\.{\\ifmmode} primitive@>
//! primitive("ifinner",if_test,if_inner_code);
//! @!@:if_inner_}{\.{\\ifinner} primitive@>
//! primitive("ifvoid",if_test,if_void_code);
//! @!@:if_void_}{\.{\\ifvoid} primitive@>
//! primitive("ifhbox",if_test,if_hbox_code);
//! @!@:if_hbox_}{\.{\\ifhbox} primitive@>
//! primitive("ifvbox",if_test,if_vbox_code);
//! @!@:if_vbox_}{\.{\\ifvbox} primitive@>
//! primitive("ifx",if_test,ifx_code);
//! @!@:ifx_}{\.{\\ifx} primitive@>
//! primitive("ifeof",if_test,if_eof_code);
//! @!@:if_eof_}{\.{\\ifeof} primitive@>
//! primitive("iftrue",if_test,if_true_code);
//! @!@:if_true_}{\.{\\iftrue} primitive@>
//! primitive("iffalse",if_test,if_false_code);
//! @!@:if_false_}{\.{\\iffalse} primitive@>
//! primitive("ifcase",if_test,if_case_code);
//! @!@:if_case_}{\.{\\ifcase} primitive@>
//!
//! @ @<Cases of |print_cmd_chr|...@>=
//! if_test: case chr_code of
//!   if_cat_code:print_esc("ifcat");
//!   if_int_code:print_esc("ifnum");
//!   if_dim_code:print_esc("ifdim");
//!   if_odd_code:print_esc("ifodd");
//!   if_vmode_code:print_esc("ifvmode");
//!   if_hmode_code:print_esc("ifhmode");
//!   if_mmode_code:print_esc("ifmmode");
//!   if_inner_code:print_esc("ifinner");
//!   if_void_code:print_esc("ifvoid");
//!   if_hbox_code:print_esc("ifhbox");
//!   if_vbox_code:print_esc("ifvbox");
//!   ifx_code:print_esc("ifx");
//!   if_eof_code:print_esc("ifeof");
//!   if_true_code:print_esc("iftrue");
//!   if_false_code:print_esc("iffalse");
//!   if_case_code:print_esc("ifcase");
//!   othercases print_esc("if")
//!   endcases;
//!
//! @ Conditions can be inside conditions, and this nesting has a stack
//! that is independent of the |save_stack|.
//!
//! Four global variables represent the top of the condition stack:
//! |cond_ptr| points to pushed-down entries, if any; |if_limit| specifies
//! the largest code of a |fi_or_else| command that is syntactically legal;
//! |cur_if| is the name of the current type of conditional; and |if_line|
//! is the line number at which it began.
//!
//! If no conditions are currently in progress, the condition stack has the
//! special state |cond_ptr=null|, |if_limit=normal|, |cur_if=0|, |if_line=0|.
//! Otherwise |cond_ptr| points to a two-word node; the |type|, |subtype|, and
//! |link| fields of the first word contain |if_limit|, |cur_if|, and
//! |cond_ptr| at the next level, and the second word contains the
//! corresponding |if_line|.
//!
//! @d if_node_size=2 {number of words in stack entry for conditionals}
//! @d if_line_field(#)==mem[#+1].int
//! @d if_code=1 {code for \.{\\if...} being evaluated}
//! @d fi_code=2 {code for \.{\\fi}}
//! @d else_code=3 {code for \.{\\else}}
//! @d or_code=4 {code for \.{\\or}}
//!
//! @<Glob...@>=
//! @!cond_ptr:pointer; {top of the condition stack}
//! @!if_limit:normal..or_code; {upper bound on |fi_or_else| codes}
//! @!cur_if:small_number; {type of conditional being worked on}
//! @!if_line:integer; {line where that conditional began}
//!
//! @ @<Set init...@>=
//! cond_ptr:=null; if_limit:=normal; cur_if:=0; if_line:=0;
//!
//! @ @<Put each...@>=
//! primitive("fi",fi_or_else,fi_code);
//! @!@:fi_}{\.{\\fi} primitive@>
//! text(frozen_fi):="fi"; eqtb[frozen_fi]:=eqtb[cur_val];
//! primitive("or",fi_or_else,or_code);
//! @!@:or_}{\.{\\or} primitive@>
//! primitive("else",fi_or_else,else_code);
//! @!@:else_}{\.{\\else} primitive@>
//!
//! @ @<Cases of |print_cmd_chr|...@>=
//! fi_or_else: if chr_code=fi_code then print_esc("fi")
//!   else if chr_code=or_code then print_esc("or")
//!   else print_esc("else");
//!
//! @ When we skip conditional text, we keep track of the line number
//! where skipping began, for use in error messages.
//!
//! @<Glob...@>=
//! @!skip_line:integer; {skipping began here}
//!
//! @ Here is a procedure that ignores text until coming to an \.{\\or},
//! \.{\\else}, or \.{\\fi} at level zero of $\.{\\if}\ldots\.{\\fi}$
//! nesting. After it has acted, |cur_chr| will indicate the token that
//! was found, but |cur_tok| will not be set (because this makes the
//! procedure run faster).
//!
//! @p procedure pass_text;
//! label done;
//! var l:integer; {level of $\.{\\if}\ldots\.{\\fi}$ nesting}
//! @!save_scanner_status:small_number; {|scanner_status| upon entry}
//! begin save_scanner_status:=scanner_status; scanner_status:=skipping; l:=0;
//! skip_line:=line;
//! loop@+  begin get_next;
//!   if cur_cmd=fi_or_else then
//!     begin if l=0 then goto done;
//!     if cur_chr=fi_code then decr(l);
//!     end
//!   else if cur_cmd=if_test then incr(l);
//!   end;
//! done: scanner_status:=save_scanner_status;
//! end;
//!
//! @ When we begin to process a new \.{\\if}, we set |if_limit:=if_code|; then
//! if\/ \.{\\or} or \.{\\else} or \.{\\fi} occurs before the current \.{\\if}
//! condition has been evaluated, \.{\\relax} will be inserted.
//! For example, a sequence of commands like `\.{\\ifvoid1\\else...\\fi}'
//! would otherwise require something after the `\.1'.
//!
//! @<Push the condition stack@>=
//! begin p:=get_node(if_node_size); link(p):=cond_ptr; type(p):=if_limit;
//! subtype(p):=cur_if; if_line_field(p):=if_line;
//! cond_ptr:=p; cur_if:=cur_chr; if_limit:=if_code; if_line:=line;
//! end
//!
//! @ @<Pop the condition stack@>=
//! begin p:=cond_ptr; if_line:=if_line_field(p);
//! cur_if:=subtype(p); if_limit:=type(p); cond_ptr:=link(p);
//! free_node(p,if_node_size);
//! end
//!
//! @ Here's a procedure that changes the |if_limit| code corresponding to
//! a given value of |cond_ptr|.
//!
//! @p procedure change_if_limit(@!l:small_number;@!p:pointer);
//! label exit;
//! var q:pointer;
//! begin if p=cond_ptr then if_limit:=l {that's the easy case}
//! else  begin q:=cond_ptr;
//!   loop@+  begin if q=null then confusion("if");
//! @:this can't happen if}{\quad if@>
//!     if link(q)=p then
//!       begin type(q):=l; return;
//!       end;
//!     q:=link(q);
//!     end;
//!   end;
//! exit:end;
//!
//! @ A condition is started when the |expand| procedure encounters
//! an |if_test| command; in that case |expand| reduces to |conditional|,
//! which is a recursive procedure.
//! @^recursion@>
//!
//! @p procedure conditional;
//! label exit,common_ending;
//! var b:boolean; {is the condition true?}
//! @!r:"<"..">"; {relation to be evaluated}
//! @!m,@!n:integer; {to be tested against the second operand}
//! @!p,@!q:pointer; {for traversing token lists in \.{\\ifx} tests}
//! @!save_scanner_status:small_number; {|scanner_status| upon entry}
//! @!save_cond_ptr:pointer; {|cond_ptr| corresponding to this conditional}
//! @!this_if:small_number; {type of this conditional}
//! begin @<Push the condition stack@>;@+save_cond_ptr:=cond_ptr;this_if:=cur_chr;@/
//! @<Either process \.{\\ifcase} or set |b| to the value of a boolean condition@>;
//! if tracing_commands>1 then @<Display the value of |b|@>;
//! if b then
//!   begin change_if_limit(else_code,save_cond_ptr);
//!   return; {wait for \.{\\else} or \.{\\fi}}
//!   end;
//! @<Skip to \.{\\else} or \.{\\fi}, then |goto common_ending|@>;
//! common_ending: if cur_chr=fi_code then @<Pop the condition stack@>
//! else if_limit:=fi_code; {wait for \.{\\fi}}
//! exit:end;
//!
//! @ In a construction like `\.{\\if\\iftrue abc\\else d\\fi}', the first
//! \.{\\else} that we come to after learning that the \.{\\if} is false is
//! not the \.{\\else} we're looking for. Hence the following curious
//! logic is needed.
//!
//! @ @<Skip to \.{\\else} or \.{\\fi}...@>=
//! loop@+  begin pass_text;
//!   if cond_ptr=save_cond_ptr then
//!     begin if cur_chr<>or_code then goto common_ending;
//!     print_err("Extra "); print_esc("or");
//! @.Extra \\or@>
//!     help1("I'm ignoring this; it doesn't match any \if.");
//!     error;
//!     end
//!   else if cur_chr=fi_code then @<Pop the condition stack@>;
//!   end
//!
//! @ @<Either process \.{\\ifcase} or set |b|...@>=
//! case this_if of
//! if_char_code, if_cat_code: @<Test if two characters match@>;
//! if_int_code, if_dim_code: @<Test relation between integers or dimensions@>;
//! if_odd_code: @<Test if an integer is odd@>;
//! if_vmode_code: b:=(abs(mode)=vmode);
//! if_hmode_code: b:=(abs(mode)=hmode);
//! if_mmode_code: b:=(abs(mode)=mmode);
//! if_inner_code: b:=(mode<0);
//! if_void_code, if_hbox_code, if_vbox_code: @<Test box register status@>;
//! ifx_code: @<Test if two tokens match@>;
//! if_eof_code: begin scan_four_bit_int; b:=(read_open[cur_val]=closed);
//!   end;
//! if_true_code: b:=true;
//! if_false_code: b:=false;
//! if_case_code: @<Select the appropriate case
//!   and |return| or |goto common_ending|@>;
//! end {there are no other cases}
//!
//! @ @<Display the value of |b|@>=
//! begin begin_diagnostic;
//! if b then print("{true}")@+else print("{false}");
//! end_diagnostic(false);
//! end
//!
//! @ Here we use the fact that |"<"|, |"="|, and |">"| are consecutive ASCII
//! codes.
//! @^ASCII code@>
//!
//! @<Test relation between integers or dimensions@>=
//! begin if this_if=if_int_code then scan_int@+else scan_normal_dimen;
//! n:=cur_val; @<Get the next non-blank non-call...@>;
//! if (cur_tok>=other_token+"<")and(cur_tok<=other_token+">") then
//!   r:=cur_tok-other_token
//! else  begin print_err("Missing = inserted for ");
//! @.Missing = inserted@>
//!   print_cmd_chr(if_test,this_if);
//!   help1("I was expecting to see `<', `=', or `>'. Didn't.");
//!   back_error; r:="=";
//!   end;
//! if this_if=if_int_code then scan_int@+else scan_normal_dimen;
//! case r of
//! "<": b:=(n<cur_val);
//! "=": b:=(n=cur_val);
//! ">": b:=(n>cur_val);
//! end;
//! end
//!
//! @ @<Test if an integer is odd@>=
//! begin scan_int; b:=odd(cur_val);
//! end
//!
//! @ @<Test box register status@>=
//! begin scan_eight_bit_int; p:=box(cur_val);
//! if this_if=if_void_code then b:=(p=null)
//! else if p=null then b:=false
//! else if this_if=if_hbox_code then b:=(type(p)=hlist_node)
//! else b:=(type(p)=vlist_node);
//! end
//!
//! @ An active character will be treated as category 13 following
//! \.{\\if\\noexpand} or following \.{\\ifcat\\noexpand}. We use the fact that
//! active characters have the smallest tokens, among all control sequences.
//!
//! @d get_x_token_or_active_char==@t@>@;
//!   begin get_x_token;
//!   if cur_cmd=relax then if cur_chr=no_expand_flag then
//!     begin cur_cmd:=active_char;
//!     cur_chr:=cur_tok-cs_token_flag-active_base;
//!     end;
//!   end
//!
//! @<Test if two characters match@>=
//! begin get_x_token_or_active_char;
//! if (cur_cmd>active_char)or(cur_chr>255) then {not a character}
//!   begin m:=relax; n:=256;
//!   end
//! else  begin m:=cur_cmd; n:=cur_chr;
//!   end;
//! get_x_token_or_active_char;
//! if (cur_cmd>active_char)or(cur_chr>255) then
//!   begin cur_cmd:=relax; cur_chr:=256;
//!   end;
//! if this_if=if_char_code then b:=(n=cur_chr)@+else b:=(m=cur_cmd);
//! end
//!
//! @ Note that `\.{\\ifx}' will declare two macros different if one is \\{long}
//! or \\{outer} and the other isn't, even though the texts of the macros are
//! the same.
//!
//! We need to reset |scanner_status|, since \.{\\outer} control sequences
//! are allowed, but we might be scanning a macro definition or preamble.
//!
//! @<Test if two tokens match@>=
//! begin save_scanner_status:=scanner_status; scanner_status:=normal;
//! get_next; n:=cur_cs; p:=cur_cmd; q:=cur_chr;
//! get_next; if cur_cmd<>p then b:=false
//! else if cur_cmd<call then b:=(cur_chr=q)
//! else @<Test if two macro texts match@>;
//! scanner_status:=save_scanner_status;
//! end
//!
//! @ Note also that `\.{\\ifx}' decides that macros \.{\\a} and \.{\\b} are
//! different in examples like this:
//! $$\vbox{\halign{\.{#}\hfil&\qquad\.{#}\hfil\cr
//!   {}\\def\\a\{\\c\}&
//!   {}\\def\\c\{\}\cr
//!   {}\\def\\b\{\\d\}&
//!   {}\\def\\d\{\}\cr}}$$
//!
//! @<Test if two macro texts match@>=
//! begin p:=link(cur_chr); q:=link(equiv(n)); {omit reference counts}
//! if p=q then b:=true
//! else begin while (p<>null)and(q<>null) do
//!     if info(p)<>info(q) then p:=null
//!     else  begin p:=link(p); q:=link(q);
//!       end;
//!   b:=((p=null)and(q=null));
//!   end;
//! end
//!
//! @ @<Select the appropriate case and |return| or |goto common_ending|@>=
//! begin scan_int; n:=cur_val; {|n| is the number of cases to pass}
//! if tracing_commands>1 then
//!   begin begin_diagnostic; print("{case "); print_int(n); print_char("}");
//!   end_diagnostic(false);
//!   end;
//! while n<>0 do
//!   begin pass_text;
//!   if cond_ptr=save_cond_ptr then
//!     if cur_chr=or_code then decr(n)
//!     else goto common_ending
//!   else if cur_chr=fi_code then @<Pop the condition stack@>;
//!   end;
//! change_if_limit(or_code,save_cond_ptr);
//! return; {wait for \.{\\or}, \.{\\else}, or \.{\\fi}}
//! end
//!
//! @ The processing of conditionals is complete except for the following
//! code, which is actually part of |expand|. It comes into play when
//! \.{\\or}, \.{\\else}, or \.{\\fi} is scanned.
//!
//! @<Terminate the current conditional and skip to \.{\\fi}@>=
//! if cur_chr>if_limit then
//!   if if_limit=if_code then insert_relax {condition not yet evaluated}
//!   else  begin print_err("Extra "); print_cmd_chr(fi_or_else,cur_chr);
//! @.Extra \\or@>
//! @.Extra \\else@>
//! @.Extra \\fi@>
//!     help1("I'm ignoring this; it doesn't match any \if.");
//!     error;
//!     end
//! else  begin while cur_chr<>fi_code do pass_text; {skip to \.{\\fi}}
//!   @<Pop the condition stack@>;
//!   end
//!
