//! @ The following procedure prints \TeX's last words before dying.
//
// @d succumb==begin if interaction=error_stop_mode then
pub(crate) fn succumb(globals: &mut TeXGlobals) -> TeXResult<!> {
    if globals.interaction == error_stop_mode {
        //   interaction:=scroll_mode; {no more interaction}
        /// no more interaction
        {
            globals.interaction = scroll_mode.into();
        }
    }
    // if log_opened then error;
    if globals.log_opened {
        error(globals)?;
    }
    // @!debug if interaction>batch_mode then debug_help;@+gubed@;@/
    crate::region_debug! {
        if globals.interaction > batch_mode {
            debug_help(globals);
            use crate::section_1338::debug_help;
        }
    }
    // history:=fatal_error_stop; jump_out; {irrecoverable error}
    globals.history = history_kind::fatal_error_stop;
    /// irrecoverable error
    jump_out()?;
    // end
    crate::never_nojump!()
}
//
// @<Error hand...@>=
// procedure fatal_error(@!s:str_number); {prints |s|, and that's it}
/// prints `s`, and that's it
pub(crate) fn fatal_error(globals: &mut TeXGlobals, s: str_number) -> TeXResult<()> {
    // begin normalize_selector;@/
    normalize_selector(globals);
    // print_err("Emergency stop"); help1(s); succumb;
    print_err!(globals, crate::strpool_str!("Emergency stop"));
    help1!(globals, s);
    succumb(globals)?;
    // @.Emergency stop@>
    // end;
    crate::return_nojump!();
}

use crate::section_0004::TeXGlobals;
use crate::section_0038::str_number;
use crate::section_0073::batch_mode;
use crate::section_0073::error_stop_mode;
use crate::section_0073::print_err;
use crate::section_0073::scroll_mode;
use crate::section_0076::history_kind;
use crate::section_0079::help1;
use crate::section_0081::jump_out;
use crate::section_0081::TeXResult;
use crate::section_0082::error;
use crate::section_0092::normalize_selector;
