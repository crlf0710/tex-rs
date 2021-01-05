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
