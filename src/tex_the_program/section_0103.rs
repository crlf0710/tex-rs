//! @ Conversely, here is a procedure analogous to |print_int|. If the output
//! of this procedure is subsequently read by \TeX\ and converted by the
//! |round_decimals| routine above, it turns out that the original value will
//! be reproduced exactly; the ``simplest'' such decimal number is output,
//! but there is always at least one digit following the decimal point.
//!
//! The invariant relation in the \&{repeat} loop is that a sequence of
//! decimal digits yet to be printed will yield the original number if and only if
//! they form a fraction~$f$ in the range $s-\delta\L10\cdot2^{16}f<s$.
//! We can stop if and only if $f=0$ satisfies this condition; the loop will
//! terminate before $s$ can possibly become zero.

// @p procedure print_scaled(@!s:scaled); {prints scaled real, rounded to five
//   digits}
/// prints scaled real, rounded to five digits
#[allow(unused_variables)]
pub(crate) fn print_scaled(globals: &mut TeXGlobals, mut s: scaled) {
    // var delta:scaled; {amount of allowable inaccuracy}
    /// amount of allowable inaccuracy
    let mut delta: integer;
    // begin if s<0 then
    if s < scaled::zero() {
        // begin print_char("-"); negate(s); {print the sign, if negative}
        /// print the sign, if negative
        const _: () = ();
        print_char(
            make_globals_io_string_log_view!(globals),
            ASCII_code_literal!(b'-'),
        );
        negate!(s);
        // end;
    }
    // print_int(s div unity); {print the integer part}
    /// print the integer part
    print_int(globals, s.inner() / unity.inner());
    // print_char(".");
    print_char(
        make_globals_io_string_log_view!(globals),
        ASCII_code_literal!(b'.'),
    );
    // s:=10*(s mod unity)+5; delta:=10;
    s = scaled::new_from_inner(10 * (s.inner() % unity.inner()) + 5);
    delta = 10;
    // repeat if delta>unity then s:=s+@'100000-50000; {round the last digit}
    loop {
        if delta > unity.inner() {
            s = scaled::new_from_inner(s.inner() + 0o100000 - 50000);
        }
        // print_char("0"+(s div unity)); s:=10*(s mod unity); delta:=delta*10;
        print_char(
            make_globals_io_string_log_view!(globals),
            ASCII_code_literal!(b'0' + (s.inner() / unity.inner()) as u8),
        );
        s = scaled::new_from_inner(10 * (s.inner() % unity.inner()));
        delta = delta * 10;
        // until s<=delta;
        if s.inner() <= delta {
            break;
        }
    }
    // end;
}

use crate::pascal::integer;
use crate::section_0004::make_globals_io_string_log_view;
use crate::section_0004::TeXGlobals;
use crate::section_0004::TeXGlobalsIoStringLogView;
use crate::section_0016::negate;
use crate::section_0018::ASCII_code_literal;
use crate::section_0058::print_char;
use crate::section_0065::print_int;
use crate::section_0101::scaled;
use crate::section_0101::unity;
