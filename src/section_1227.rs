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
