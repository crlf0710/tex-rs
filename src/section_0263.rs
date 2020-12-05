//! @ Here is a similar procedure; it avoids the error checks, and it never
//! prints a space after the control sequence.
//
// @<Basic printing procedures@>=
// procedure sprint_cs(@!p:pointer); {prints a control sequence}
/// prints a control sequence
pub(crate) fn sprint_cs(globals: &mut TeXGlobals, p: pointer) {
    // begin if p<hash_base then
    if p < hash_base as pointer {
        todo!();
        // if p<single_base then print(p-active_base)
        // else  if p<null_cs then print_esc(p-single_base)
        //   else  begin print_esc("csname"); print_esc("endcsname");
        //     end
    }
    // else print_esc(text(p));
    else {
        print_esc(globals, str_number::new(text!(globals, p) as _));
    }
    // end;
}

use crate::section_0004::TeXGlobals;
use crate::section_0038::str_number;
use crate::section_0063::print_esc;
use crate::section_0115::pointer;
use crate::section_0222::hash_base;
