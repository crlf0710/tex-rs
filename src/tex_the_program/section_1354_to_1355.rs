//! @ When `\.{\\special\{...\}}' appears, we expand the macros in the token
//! list as in \.{\\xdef} and \.{\\mark}.
//!
//! @<Implement \.{\\special}@>=
//! begin new_whatsit(special_node,write_node_size); write_stream(tail):=null;
//! p:=scan_toks(false,true); write_tokens(tail):=def_ref;
//! end
//!
//! @ Each new type of node that appears in our data structure must be capable
//! of being displayed, copied, destroyed, and so on. The routines that we
//! need for write-oriented whatsits are somewhat like those for mark nodes;
//! other extensions might, of course, involve more subtlety here.
//!
//! @<Basic printing...@>=
//! procedure print_write_whatsit(@!s:str_number;@!p:pointer);
//! begin print_esc(s);
//! if write_stream(p)<16 then print_int(write_stream(p))
//! else if write_stream(p)=16 then print_char("*")
//! @.*\relax@>
//! else print_char("-");
//! end;
//!
