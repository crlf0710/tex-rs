//! @ @<Copy the token list@>=
//! begin p:=temp_head; link(p):=null;
//! if cur_val_level=ident_val then store_new_token(cs_token_flag+cur_val)
//! else if cur_val<>null then
//!   begin r:=link(cur_val); {do not copy the reference count}
//!   while r<>null do
//!     begin fast_store_new_token(info(r)); r:=link(r);
//!     end;
//!   end;
//! the_toks:=p;
//! end
//!
