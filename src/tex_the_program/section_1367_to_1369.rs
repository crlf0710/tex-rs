//! @ @<Output the whatsit node |p| in an hlist@>=
//! out_what(p)
//!
//! @ After all this preliminary shuffling, we come finally to the routines
//! that actually send out the requested data. Let's do \.{\\special} first
//! (it's easier).
//!
//! @<Declare procedures needed in |hlist_out|, |vlist_out|@>=
//! procedure special_out(@!p:pointer);
//! var old_setting:0..max_selector; {holds print |selector|}
//! @!k:pool_pointer; {index into |str_pool|}
//! begin synch_h; synch_v;@/
//! old_setting:=selector; selector:=new_string;
//! show_token_list(link(write_tokens(p)),null,pool_size-pool_ptr);
//! selector:=old_setting;
//! str_room(1);
//! if cur_length<256 then
//!   begin dvi_out(xxx1); dvi_out(cur_length);
//!   end
//! else  begin dvi_out(xxx4); dvi_four(cur_length);
//!   end;
//! for k:=str_start[str_ptr] to pool_ptr-1 do dvi_out(so(str_pool[k]));
//! pool_ptr:=str_start[str_ptr]; {erase the string}
//! end;
//!
//! @ To write a token list, we must run it through \TeX's scanner, expanding
//! macros and \.{\\the} and \.{\\number}, etc. This might cause runaways,
//! if a delimited macro parameter isn't matched, and runaways would be
//! extremely confusing since we are calling on \TeX's scanner in the middle
//! of a \.{\\shipout} command. Therefore we will put a dummy control sequence as
//! a ``stopper,'' right after the token list. This control sequence is
//! artificially defined to be \.{\\outer}.
//! @:end_write_}{\.{\\endwrite}@>
//!
//! @<Initialize table...@>=
//! text(end_write):="endwrite"; eq_level(end_write):=level_one;
//! eq_type(end_write):=outer_call; equiv(end_write):=null;
//!
