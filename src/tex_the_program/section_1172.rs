//! @ The routine that scans the four mlists of a \.{\\mathchoice} is very
//! much like the routine that builds discretionary nodes.
//!
//! @<Declare act...@>=
//! procedure append_choices;
//! begin tail_append(new_choice); incr(save_ptr); saved(-1):=0;
//! push_math(math_choice_group); scan_left_brace;
//! end;
//!
