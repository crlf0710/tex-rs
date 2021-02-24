//! @* \[48] Building math lists.
//! The routines that \TeX\ uses to create mlists are similar to those we have
//! just seen for the generation of hlists and vlists. But it is necessary to
//! make ``noads'' as well as nodes, so the reader should review the
//! discussion of math mode data structures before trying to make sense out of
//! the following program.
//!
//! Here is a little routine that needs to be done whenever a subformula
//! is about to be processed. The parameter is a code like |math_group|.
//!
//! @<Declare act...@>=
//! procedure push_math(@!c:group_code);
//! begin push_nest; mode:=-mmode; incompleat_noad:=null; new_save_level(c);
//! end;
//!
