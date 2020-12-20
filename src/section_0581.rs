//! @ When \TeX\ wants to typeset a character that doesn't exist, the
//! character node is not created; thus the output routine can assume
//! that characters exist when it sees them. The following procedure
//! prints a warning message unless the user has suppressed it.
//
// @p procedure char_warning(@!f:internal_font_number;@!c:eight_bits);
#[allow(unused_variables)]
pub(crate) fn char_warning(globals: &mut TeXGlobals, f: internal_font_number, c: ASCII_code) {
    // begin if tracing_lost_chars>0 then
    if tracing_lost_chars!(globals) > 0 {
        // begin begin_diagnostic;
        begin_diagnostic(globals);
        // print_nl("Missing character: There is no ");
        print_nl(globals, strpool_str!("Missing character: There is no "));
        // @.Missing character@>
        // print_ASCII(c); print(" in font ");
        print_ASCII(globals, c.numeric_value() as _);
        print(globals, strpool_str!(" in font ").get() as _);
        // slow_print(font_name[f]); print_char("!"); end_diagnostic(false);
        let font_name_str =  font_name_str(globals, f);
        slow_print(globals, font_name_str.get() as _);
        print_char(make_globals_io_string_log_view!(globals), ASCII_code_literal!(b'!'));
        end_diagnostic(globals, false);
        // end;
    }
    // end;
}

use crate::section_0004::TeXGlobals;
use crate::section_0004::TeXGlobalsIoStringLogView;
use crate::section_0018::ASCII_code;
use crate::section_0058::print_char;
use crate::section_0059::print;
use crate::section_0060::slow_print;
use crate::section_0062::print_nl;
use crate::section_0068::print_ASCII;
use crate::section_0245::begin_diagnostic;
use crate::section_0245::end_diagnostic;
use crate::section_0548::internal_font_number;
use crate::section_0549::font_name_str;
