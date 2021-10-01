//! @ @<Display mark |p|@>=
//! begin print_esc("mark"); print_mark(mark_ptr(p));
//! end
//!
//! @ @<Display adjustment |p|@>=
//! begin print_esc("vadjust"); node_list_display(adjust_ptr(p)); {recursive call}
//! end
//!
