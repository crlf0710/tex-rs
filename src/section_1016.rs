//! @ @<Update the values of |first_mark| and |bot_mark|@>=
//! begin if first_mark=null then
//!   begin first_mark:=mark_ptr(p);
//!   add_token_ref(first_mark);
//!   end;
//! if bot_mark<>null then delete_token_ref(bot_mark);
//! bot_mark:=mark_ptr(p); add_token_ref(bot_mark);
//! end
//!
