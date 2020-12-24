//! @ The implementation of \.{\\noexpand} is a bit trickier, because it is
//! necessary to insert a special `|dont_expand|' marker into \TeX's reading
//! mechanism.  This special marker is processed by |get_next|, but it does
//! not slow down the inner loop.
//!
//! Since \.{\\outer} macros might arise here, we must also
//! clear the |scanner_status| temporarily.
//!
//! @<Suppress expansion...@>=
//! begin save_scanner_status:=scanner_status; scanner_status:=normal;
//! get_token; scanner_status:=save_scanner_status; t:=cur_tok;
//! back_input; {now |start| and |loc| point to the backed-up token |t|}
//! if t>=cs_token_flag then
//!   begin p:=get_avail; info(p):=cs_token_flag+frozen_dont_expand;
//!   link(p):=loc; start:=p; loc:=p;
//!   end;
//! end
//!
