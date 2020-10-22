//! @ The following procedure prints \TeX's last words before dying.
//
// @d succumb==begin if interaction=error_stop_mode then
//     interaction:=scroll_mode; {no more interaction}
//   if log_opened then error;
//   @!debug if interaction>batch_mode then debug_help;@+gubed@;@/
//   history:=fatal_error_stop; jump_out; {irrecoverable error}
//   end
//
// @<Error hand...@>=
// procedure fatal_error(@!s:str_number); {prints |s|, and that's it}
/// prints `s`, and that's it
#[allow(unused_variables)]
pub(crate) fn fatal_error(
    globals: &mut TeXGlobals,
    s: str_number,
) -> Result<(), JumpOutToEndOfTEX> {
    // begin normalize_selector;@/
    // print_err("Emergency stop"); help1(s); succumb;
    // @.Emergency stop@>
    // end;
    return_nojump!();
}

use crate::section_0004::TeXGlobals;
use crate::section_0038::str_number;
use crate::section_0081::JumpOutToEndOfTEX;
