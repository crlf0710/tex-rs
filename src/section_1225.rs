//! @ @<Assignments@>=
//! read_to_cs: begin scan_int; n:=cur_val;
//!   if not scan_keyword("to") then
//! @.to@>
//!     begin print_err("Missing `to' inserted");
//! @.Missing `to'...@>
//!     help2("You should have said `\read<number> to \cs'.")@/
//!     ("I'm going to look for the \cs now."); error;
//!     end;
//!   get_r_token;
//!   p:=cur_cs; read_toks(n,p); define(p,call,cur_val);
//!   end;
//!
