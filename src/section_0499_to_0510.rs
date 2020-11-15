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
