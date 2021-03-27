//! @ Here is the most dreaded error message.
//
// @<Error hand...@>=
// procedure overflow(@!s:str_number;@!n:integer); {stop due to finiteness}
/// stop due to finiteness
#[allow(unused_variables)]
pub(crate) fn overflow(globals: &mut TeXGlobals, s: str_number, n: integer) -> TeXResult<()> {
    // begin normalize_selector;
    // print_err("TeX capacity exceeded, sorry [");
    // @.TeX capacity exceeded ...@>
    // print(s); print_char("="); print_int(n); print_char("]");
    // help2("If you really absolutely need more capacity,")@/
    //   ("you can ask a wizard to enlarge me.");
    // succumb;
    // end;
    todo!();
}

use crate::pascal::integer;
use crate::section_0004::TeXGlobals;
use crate::section_0038::str_number;
use crate::section_0081::TeXResult;
