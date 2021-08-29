//! @ An array of digits in the range |0..15| is printed by |print_the_digs|.
//
// @<Basic print...@>=
// procedure print_the_digs(@!k:eight_bits);
//   {prints |dig[k-1]|$\,\ldots\,$|dig[0]|}
/// prints `dig[k-1]`...`dig[0]`
pub(crate) fn print_the_digs(globals: &mut TeXGlobals, mut k: eight_bits) {
    // begin while k>0 do
    while k > 0 {
        // begin decr(k);
        decr!(k);
        // if dig[k]<10 then print_char("0"+dig[k])
        if globals.dig[k as usize] < 10 {
            print_char(
                make_globals_io_string_log_view!(globals),
                ASCII_code_literal!(b'0' + globals.dig[k as usize].get()),
            );
        }
        // else print_char("A"-10+dig[k]);
        else {
            print_char(
                make_globals_io_string_log_view!(globals),
                ASCII_code_literal!(b'A' - 10 + globals.dig[k as usize].get()),
            );
        }
        // end;
    }
    // end;
}

use crate::section_0004::TeXGlobals;
use crate::section_0004::TeXGlobalsIoStringLogView;
use crate::section_0025::eight_bits;
use crate::section_0058::print_char;
