//! @ @<Report an improper use...@>=
//! begin print_err("Use of "); sprint_cs(warning_index);
//! @.Use of x doesn't match...@>
//! print(" doesn't match its definition");
//! help4("If you say, e.g., `\def\a1{...}', then you must always")@/
//!   ("put `1' after `\a', since control sequence names are")@/
//!   ("made up of letters only. The macro here has not been")@/
//!   ("followed by the required stuff, so I'm ignoring it.");
//! error; return;
//! end
//!
//! @ @<Contribute an entire group to the current parameter@>=
//! begin unbalance:=1;
//! @^inner loop@>
//! loop@+  begin fast_store_new_token(cur_tok); get_token;
//!   if cur_tok=par_token then if long_state<>long_call then
//!     @<Report a runaway argument and abort@>;
//!   if cur_tok<right_brace_limit then
//!     if cur_tok<left_brace_limit then incr(unbalance)
//!     else  begin decr(unbalance);
//!       if unbalance=0 then goto done1;
//!       end;
//!   end;
//! done1: rbrace_ptr:=p; store_new_token(cur_tok);
//! end
//!
