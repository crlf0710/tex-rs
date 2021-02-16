//! @ The |post_break| list of a discretionary node is indicated by a prefixed
//! `\.{\char'174}' instead of the `\..' before the |pre_break| list.
//!
//! @<Display discretionary |p|@>=
//! begin print_esc("discretionary");
//! if replace_count(p)>0 then
//!   begin print(" replacing "); print_int(replace_count(p));
//!   end;
//! node_list_display(pre_break(p)); {recursive call}
//! append_char("|"); show_node_list(post_break(p)); flush_char; {recursive call}
//! end
//!
//! @ @<Display mark |p|@>=
//! begin print_esc("mark"); print_mark(mark_ptr(p));
//! end
//!
//! @ @<Display adjustment |p|@>=
//! begin print_esc("vadjust"); node_list_display(adjust_ptr(p)); {recursive call}
//! end
//!
