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
//! texts.
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
/// prints the mode represented by `m`
#[allow(unused_variables)]
pub(crate) fn print_mode(globals: &mut TeXGlobals, m: integer) {
    // begin if m>0 then
    if m > 0 {
        // case m div (max_command+1) of
        match m / (max_command + 1) as integer {
            // 0:print("vertical");
            0 => print(globals, strpool_str!("vertical").get() as _),
            // 1:print("horizontal");
            1 => print(globals, strpool_str!("horizontal").get() as _),
            // 2:print("display math");
            2 => print(globals, strpool_str!("display math").get() as _),
            _ => {
                trace_error_expr!("m = {}", m);
                unreachable!()
            },
        }
        // end
    }
    // else if m=0 then print("no")
    else if m == 0 {
        print(globals, strpool_str!("no").get() as _);
    }
    // else  case (-m) div (max_command+1) of
    else {
        match (-m) / (max_command + 1) as integer {
            // 0:print("internal vertical");
            0 => print(globals, strpool_str!("internal vertical").get() as _),
            // 1:print("restricted horizontal");
            1 => print(globals, strpool_str!("restricted horizontal").get() as _),
            // 2:print("math");
            2 => print(globals, strpool_str!("math").get() as _),
            _ => unreachable!(),
        }
        // end;
    }
    // print(" mode");
    print(globals, strpool_str!(" mode").get() as _);
    // end;
}

use crate::pascal::integer;
use crate::section_0004::TeXGlobals;
use crate::section_0059::print;
use crate::section_0209::max_command_POS_TYPENUM;
use crate::section_0209::max_command;
use typenum::Integer;
use typenum::{N1, P1};

migration_complete!();
