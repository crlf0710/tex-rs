//! @ \TeX\ has eight procedures that govern alignments: |init_align| and
//! |fin_align| are used at the very beginning and the very end; |init_row| and
//! |fin_row| are used at the beginning and end of individual rows; |init_span|
//! is used at the beginning of a sequence of spanned columns (possibly involving
//! only one column); |init_col| and |fin_col| are used at the beginning and
//! end of individual columns; and |align_peek| is used after \.{\\cr} to see
//! whether the next item is \.{\\noalign}.
//!
//! We shall consider these routines in the order they are first used during
//! the course of a complete \.{\\halign}, namely |init_align|, |align_peek|,
//! |init_row|, |init_span|, |init_col|, |fin_col|, |fin_row|, |fin_align|.
//!
