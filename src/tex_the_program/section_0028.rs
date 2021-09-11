//! @ Files can be closed with the \ph\ routine `|close(f)|', which
//! @:PASCAL H}{\ph@>
//! @^system dependencies@>
//! should be used when all input or output with respect to |f| has been completed.
//! This makes |f| available to be opened again, if desired; and if |f| was used for
//! output, the |close| operation makes the corresponding external file appear
//! on the user's area, ready to be read.
//!
//! These procedures should not generate error messages if a file is
//! being closed before it has been successfully opened.
//
// @p procedure a_close(var f:alpha_file); {close a text file}
/// close a text file
pub(crate) fn a_close(f: &mut alpha_file) {
    // begin close(f);
    close(f);
    // end;
}
// @#
// procedure b_close(var f:byte_file); {close a binary file}
/// close a binary file
pub(crate) fn b_close(f: &mut byte_file) {
    // begin close(f);
    close(f);
    // end;
}
// @#
// procedure w_close(var f:word_file); {close a word file}
/// close a word file
pub(crate) fn w_close(f: &mut word_file) {
    // begin close(f);
    close(f);
    // end;
}

use crate::io_support::close;
use crate::section_0025::alpha_file;
use crate::section_0025::byte_file;
use crate::section_0113::word_file;
