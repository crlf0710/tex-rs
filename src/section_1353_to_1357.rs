
//! @ @<Implement \.{\\closeout}@>=
//! begin new_write_whatsit(write_node_size); write_tokens(tail):=null;
//! end
//!
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
//! @ @<Display the whatsit...@>=
//! case subtype(p) of
//! open_node:begin print_write_whatsit("openout",p);
//!   print_char("="); print_file_name(open_name(p),open_area(p),open_ext(p));
//!   end;
//! write_node:begin print_write_whatsit("write",p);
//!   print_mark(write_tokens(p));
//!   end;
//! close_node:print_write_whatsit("closeout",p);
//! special_node:begin print_esc("special");
//!   print_mark(write_tokens(p));
//!   end;
//! language_node:begin print_esc("setlanguage");
//!   print_int(what_lang(p)); print(" (hyphenmin ");
//!   print_int(what_lhm(p)); print_char(",");
//!   print_int(what_rhm(p)); print_char(")");
//!   end;
//! othercases print("whatsit?")
//! endcases
//!
//! @ @<Make a partial copy of the whatsit...@>=
//! case subtype(p) of
//! open_node: begin r:=get_node(open_node_size); words:=open_node_size;
//!   end;
//! write_node,special_node: begin r:=get_node(write_node_size);
//!   add_token_ref(write_tokens(p)); words:=write_node_size;
//!   end;
//! close_node,language_node: begin r:=get_node(small_node_size);
//!   words:=small_node_size;
//!   end;
//! othercases confusion("ext2")
//! @:this can't happen ext2}{\quad ext2@>
//! endcases
//!