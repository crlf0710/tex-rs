//! @ Here is a procedure that uses |scanner_status| to print a warning message
//! when a subfile has ended, and at certain other crucial times:
//!
//! @<Declare the procedure called |runaway|@>=
//! procedure runaway;
//! var p:pointer; {head of runaway list}
//! begin if scanner_status>skipping then
//!   begin print_nl("Runaway ");
//! @.Runaway...@>
//!   case scanner_status of
//!   defining: begin print("definition"); p:=def_ref;
//!     end;
//!   matching: begin print("argument"); p:=temp_head;
//!     end;
//!   aligning: begin print("preamble"); p:=hold_head;
//!     end;
//!   absorbing: begin print("text"); p:=def_ref;
//!     end;
//!   end; {there are no other cases}
//!   print_char("?");print_ln; show_token_list(link(p),null,error_line-10);
//!   end;
//! end;
//!
