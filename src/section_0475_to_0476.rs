//! @ @<Express shock...@>=
//! begin print_err("Missing { inserted"); incr(align_state);
//! @.Missing \{ inserted@>
//! help2("Where was the left brace? You said something like `\def\a}',")@/
//!   ("which I'm going to interpret as `\def\a{}'."); error; goto found;
//! end
//!
//! @ @<If the next character is a parameter number...@>=
//! begin s:=match_token+cur_chr; get_token;
//! if cur_cmd=left_brace then
//!   begin hash_brace:=cur_tok;
//!   store_new_token(cur_tok); store_new_token(end_match_token);
//!   goto done;
//!   end;
//! if t=zero_token+9 then
//!   begin print_err("You already have nine parameters");
//! @.You already have nine...@>
//!   help1("I'm going to ignore the # sign you just used."); error;
//!   end
//! else  begin incr(t);
//!   if cur_tok<>t then
//!     begin print_err("Parameters must be numbered consecutively");
//! @.Parameters...consecutively@>
//!     help2("I've inserted the digit you should have used after the #.")@/
//!       ("Type `1' to delete what you did use."); back_error;
//!     end;
//!   cur_tok:=s;
//!   end;
//! end
//!
