//! @ When `\.{\\special\{...\}}' appears, we expand the macros in the token
//! list as in \.{\\xdef} and \.{\\mark}.
//!
//! @<Implement \.{\\special}@>=
//! begin new_whatsit(special_node,write_node_size); write_stream(tail):=null;
//! p:=scan_toks(false,true); write_tokens(tail):=def_ref;
//! end
//!
