//! @ Here is a similar procedure; it avoids the error checks, and it never
//! prints a space after the control sequence.
//
// @<Basic printing procedures@>=
// procedure sprint_cs(@!p:pointer); {prints a control sequence}
/// prints a control sequence
pub(crate) fn sprint_cs(globals: &mut TeXGlobals, p: pointer) {
    // begin if p<hash_base then
    if p < hash_base as pointer {
        // if p<single_base then print(p-active_base)
        if (p as word) < single_base {
            print(globals, (p as word - active_base) as _);
        }
        // else if p<null_cs then print_esc(p-single_base)
        else if (p as word) < null_cs as word {
            print_esc(globals, str_number::new((p as word - single_base) as _));
        }
        // else begin print_esc("csname"); print_esc("endcsname");
        else {
            print_esc(globals, crate::strpool_str!("csname"));
            print_esc(globals, crate::strpool_str!("endcsname"));
            // end
        }
    }
    // else print_esc(text(p));
    else {
        print_esc(globals, str_number::new(text!(globals, p) as _));
    }
    // end;
}

use crate::pascal::word;
use crate::section_0004::TeXGlobals;
use crate::section_0038::str_number;
use crate::section_0059::print;
use crate::section_0063::print_esc;
use crate::section_0115::pointer;
use crate::section_0222::active_base;
use crate::section_0222::hash_base;
use crate::section_0222::null_cs;
use crate::section_0222::single_base;
use crate::section_0256::text;
