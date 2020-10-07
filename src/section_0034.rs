//! @ Sometimes it is necessary to synchronize the input/output mixture that
//! happens on the user's terminal, and three system-dependent
//! procedures are used for this
//! purpose. The first of these, |update_terminal|, is called when we want
//! to make sure that everything we have output to the terminal so far has
//! actually left the computer's internal buffers and been sent.
//! The second, |clear_terminal|, is called when we wish to cancel any
//! input that the user may have typed ahead (since we are about to
//! issue an unexpected error message). The third, |wake_up_terminal|,
//! is supposed to revive the terminal if the user has disabled it by
//! some instruction to the operating system.  The following macros show how
//! these operations can be specified in \ph:
//! @:PASCAL H}{\ph@>
//! @^system dependencies@>

// @d update_terminal == break(term_out) {empty the terminal output buffer}

/// empty the terminal output buffer
pub(crate) fn update_terminal(globals: &mut TeXGlobals) {
    r#break(&mut globals.term_out);
}

// @d clear_terminal == break_in(term_in,true) {clear the terminal input buffer}
// @d wake_up_terminal == do_nothing {cancel the user's cancellation of output}
//

use crate::pascal::r#break;
use crate::section_0004::TeXGlobals;
