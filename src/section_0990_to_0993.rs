//! @ @<Set init...@>=
//! output_active:=false; insert_penalties:=0;
//!
//! @ The page builder is ready to start a fresh page if we initialize
//! the following state variables. (However, the page insertion list is initialized
//! elsewhere.)
//!
//! @<Start a new current page@>=
//! page_contents:=empty; page_tail:=page_head; link(page_head):=null;@/
//! last_glue:=max_halfword; last_penalty:=0; last_kern:=0;
//! page_depth:=0; page_max_depth:=0
//!
//! @ At certain times box 255 is supposed to be void (i.e., |null|),
//! or an insertion box is supposed to be ready to accept a vertical list.
//! If not, an error message is printed, and the following subroutine
//! flushes the unwanted contents, reporting them to the user.
//!
//! @p procedure box_error(@!n:eight_bits);
//! begin error; begin_diagnostic;
//! print_nl("The following box has been deleted:");
//! @.The following...deleted@>
//! show_box(box(n)); end_diagnostic(true);
//! flush_node_list(box(n)); box(n):=null;
//! end;
//!
//! @ The following procedure guarantees that a given box register
//! does not contain an \.{\\hbox}.
//!
//! @p procedure ensure_vbox(@!n:eight_bits);
//! var p:pointer; {the box register contents}
//! begin p:=box(n);
//! if p<>null then if type(p)=hlist_node then
//!   begin print_err("Insertions can only be added to a vbox");
//! @.Insertions can only...@>
//!   help3("Tut tut: You're trying to \insert into a")@/
//!     ("\box register that now contains an \hbox.")@/
//!     ("Proceed, and I'll discard its present contents.");
//!   box_error(n);
//!   end;
//! end;
//!
