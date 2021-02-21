//! ` `
// @<Declare the procedure called |restore_trace|@>=
// @!stat procedure restore_trace(@!p:pointer;@!s:str_number);
//   {|eqtb[p]| has just been restored or retained}
/// `eqtb[p]` has just been restored or retained
#[cfg(feature = "statistics")]
#[allow(unused_variables)]
pub(crate) fn restore_trace(globals: &mut TeXGlobals, p: pointer, s: str_number) {
    // begin begin_diagnostic; print_char("{"); print(s); print_char(" ");
    // show_eqtb(p); print_char("}");
    // end_diagnostic(false);
    // end;
    // tats
    todo!();
}

use crate::section_0004::TeXGlobals;
use crate::section_0115::pointer;
use crate::section_0038::str_number;