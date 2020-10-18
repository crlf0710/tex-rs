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
// begin close(f);
// end;
// @#
// procedure w_close(var f:word_file); {close a word file}
// begin close(f);
// end;
//

use crate::pascal::close;
use crate::section_0025::alpha_file;
