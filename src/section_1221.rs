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
