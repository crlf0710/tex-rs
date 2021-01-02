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
