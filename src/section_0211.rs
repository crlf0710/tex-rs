//! @* \[16] The semantic nest.
//! \TeX\ is typically in the midst of building many lists at once. For example,
//! when a math formula is being processed, \TeX\ is in math mode and
//! working on an mlist; this formula has temporarily interrupted \TeX\ from
//! being in horizontal mode and building the hlist of a paragraph; and this
//! paragraph has temporarily interrupted \TeX\ from being in vertical mode
//! and building the vlist for the next page of a document. Similarly, when a
//! \.{\\vbox} occurs inside of an \.{\\hbox}, \TeX\ is temporarily
//! interrupted from working in restricted horizontal mode, and it enters
//! internal vertical mode.  The ``semantic nest'' is a stack that
//! keeps track of what lists and modes are currently suspended.
//!
//! At each level of processing we are in one of six modes:
//!
//! \yskip\hang|vmode| stands for vertical mode (the page builder);
//!
//! \hang|hmode| stands for horizontal mode (the paragraph builder);
//!
//! \hang|mmode| stands for displayed formula mode;
//!
//! \hang|-vmode| stands for internal vertical mode (e.g., in a \.{\\vbox});
//!
//! \hang|-hmode| stands for restricted horizontal mode (e.g., in an \.{\\hbox});
//!
//! \hang|-mmode| stands for math formula mode (not displayed).
//!
//! \yskip\noindent The mode is temporarily set to zero while processing \.{\\write}
//! texts in the |ship_out| routine.
//!
//! Numeric values are assigned to |vmode|, |hmode|, and |mmode| so that
//! \TeX's ``big semantic switch'' can select the appropriate thing to
//! do by computing the value |abs(mode)+cur_cmd|, where |mode| is the current
//! mode and |cur_cmd| is the current command code.
//
// @d vmode=1 {vertical mode}
/// vertical mode
pub(crate) const vmode: i16 = vmode_POS_TYPENUM::I16;
pub(crate) type vmode_POS_TYPENUM = P1;
// @d hmode=vmode+max_command+1 {horizontal mode}
/// horizontal mode
pub(crate) const hmode: i16 = hmode_POS_TYPENUM::I16;
pub(crate) type hmode_POS_TYPENUM = typenum::op!(vmode_POS_TYPENUM + max_command_POS_TYPENUM + P1);
// @d mmode=hmode+max_command+1 {math mode}
/// math mode
pub(crate) const mmode: i16 = mmode_POS_TYPENUM::I16;
pub(crate) type mmode_POS_TYPENUM = typenum::op!(hmode_POS_TYPENUM + max_command_POS_TYPENUM + P1);

pub(crate) type mmode_NEG_TYPENUM = typenum::op!(mmode_POS_TYPENUM * N1);

// @p procedure print_mode(@!m:integer); {prints the mode represented by |m|}
// begin if m>0 then
//   case m div (max_command+1) of
//   0:print("vertical");
//   1:print("horizontal");
//   2:print("display math");
//   end
// else if m=0 then print("no")
// else  case (-m) div (max_command+1) of
//   0:print("internal vertical");
//   1:print("restricted horizontal");
//   2:print("math");
//   end;
// print(" mode");
// end;
//

use crate::section_0209::max_command_POS_TYPENUM;
use typenum::Integer;
use typenum::{N1, P1};
