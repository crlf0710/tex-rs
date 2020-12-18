//! @ In anomalous cases, the print selector might be in an unknown state;
//! the following subroutine is called to fix things just enough to keep
//! running a bit longer.
//
// @p procedure normalize_selector;
pub(crate) fn normalize_selector(globals: &mut TeXGlobals) {
    // begin if log_opened then selector:=term_and_log
    if globals.log_opened {
        globals.selector = term_and_log.into();
    }
    // else selector:=term_only;
    else {
        globals.selector = term_only.into();
    }
    // if job_name=0 then open_log_file;
    if globals.job_name == 0 {
        open_log_file(globals);
    }
    // if interaction=batch_mode then decr(selector);
    if globals.interaction == batch_mode {
        decr!(globals.selector);
    }
    // end;
}

use crate::section_0004::TeXGlobals;
use crate::section_0054::term_and_log;
use crate::section_0054::term_only;
use crate::section_0073::batch_mode;
use crate::section_0534::open_log_file;