//! @ The |jump_out| procedure just cuts across all active procedure levels and
//! goes to |end_of_TEX|. This is the only nontrivial |@!goto| statement in the
//! whole program. It is used when there is no recovery from a particular error.
//!
//! Some \PASCAL\ compilers do not implement non-local |goto| statements.
//! @^system dependencies@>
//! In such cases the body of |jump_out| should simply be
//! `|close_files_and_terminate|;\thinspace' followed by a call on some system
//! procedure that quietly terminates the program.
//
// @<Error hand...@>=
// procedure jump_out;
pub(crate) fn jump_out() -> TeXResult<()> {
    // begin goto end_of_TEX;
    return Err(JumpOutToEndOfTEX);
    // end;
}

pub(crate) struct JumpOutToEndOfTEX;

impl fmt::Display for JumpOutToEndOfTEX {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "JumpOutToEndOfTEX")
    }
}

pub(crate) type TeXResult<T> = Result<T, JumpOutToEndOfTEX>;

pub(crate) macro try_or_jump($val:expr, $jump_target:lifetime) {
    match $val {
        Ok(v) => v,
        Err(crate::section_0081::JumpOutToEndOfTEX) => crate::goto_forward_label!($jump_target),
    }
}

use core::fmt;

crate::migration_complete!();
