//! @ Here is a trivial procedure to print two digits; it is usually called with
//! a parameter in the range |0<=n<=99|.
//
// @p procedure print_two(@!n:integer); {prints two least significant digits}
/// prints two least significant digits
pub(crate) fn print_two(globals: &mut TeXGlobals, mut n: integer) {
    // begin n:=abs(n) mod 100; print_char("0"+(n div 10));
    n = n.abs() % 100;
    print_char(
        make_globals_io_string_log_view!(globals),
        ASCII_code_literal!(b'0' + (n / 10) as u8),
    );
    // print_char("0"+(n mod 10));
    print_char(
        make_globals_io_string_log_view!(globals),
        ASCII_code_literal!(b'0' + (n % 10) as u8),
    );
    // end;
}

use crate::pascal::integer;
use crate::section_0004::TeXGlobals;
use crate::section_0004::TeXGlobalsIoStringLogView;
use crate::section_0058::print_char;
