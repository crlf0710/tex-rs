//! @ When \TeX\ wants to typeset a character that doesn't exist, the
//! character node is not created; thus the output routine can assume
//! that characters exist when it sees them. The following procedure
//! prints a warning message unless the user has suppressed it.
//
// @p procedure char_warning(@!f:internal_font_number;@!c:eight_bits);
#[allow(unused_variables)]
pub(crate) fn char_warning(globals: &mut TeXGlobals, f: internal_font_number, c: ASCII_code) {
    // begin if tracing_lost_chars>0 then
    //   begin begin_diagnostic;
    //   print_nl("Missing character: There is no ");
    // @.Missing character@>
    //   print_ASCII(c); print(" in font ");
    //   slow_print(font_name[f]); print_char("!"); end_diagnostic(false);
    //   end;
    // end;
}

use crate::section_0004::TeXGlobals;
use crate::section_0548::internal_font_number;
use crate::section_0018::ASCII_code;
