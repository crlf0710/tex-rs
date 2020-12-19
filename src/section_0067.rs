//! @ Hexadecimal printing of nonnegative integers is accomplished by |print_hex|.
//
// @p procedure print_hex(@!n:integer);
//   {prints a positive integer in hexadecimal form}
/// prints a positive integer in hexadecimal form
pub(crate) fn print_hex(globals: &mut TeXGlobals, mut n: integer) {
    // var k:0..22; {index to current digit; we assume that $0\L n<16^{22}$}
    /// index to current digit; we assume that `0 <= n<16^{22}`
    let mut k;
    // begin k:=0; print_char("""");
    k = 0;
    print_char(make_globals_io_string_view!(globals), ASCII_code_literal!(b'"'));
    // repeat dig[k]:=n mod 16; n:=n div 16; incr(k);
    loop {
        globals.dig[k as usize] = ((n % 16) as u8).into();
        n = n / 16;
        incr!(k);
        // until n=0;
        if n == 0 {
            break;
        }
    }
    // print_the_digs(k);
    print_the_digs(globals, k);
    // end;
}

use crate::section_0004::TeXGlobals;
use crate::section_0004::TeXGlobalsIoStringView;
use crate::section_0058::print_char;
use crate::section_0064::print_the_digs;
use crate::pascal::integer;