//! @ The help messages here contain a little white lie, since \.{\\noalign}
//! and \.{\\omit} are allowed also after `\.{\\noalign\{...\}}'.
//!
//! @<Declare act...@>=
//! procedure no_align_error;
//! begin print_err("Misplaced "); print_esc("noalign");
//! @.Misplaced \\noalign@>
//! help2("I expect to see \noalign only after the \cr of")@/
//!   ("an alignment. Proceed, and I'll ignore this case."); error;
//! end;
//! procedure omit_error;
//! begin print_err("Misplaced "); print_esc("omit");
//! @.Misplaced \\omit@>
//! help2("I expect to see \omit only after tab marks or the \cr of")@/
//!   ("an alignment. Proceed, and I'll ignore this case."); error;
//! end;
//!
