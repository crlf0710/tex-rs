//! @ The |dvi_four| procedure outputs four bytes in two's complement notation,
//! without risking arithmetic overflow.
//
// @p procedure dvi_four(@!x:integer);
pub(crate) fn dvi_four(globals: &mut TeXGlobals, x: integer) {
    // begin if x>=0 then dvi_out(x div @'100000000)
    let mut x: word = if x >= 0 {
        dvi_out!(globals, x / 0o100000000);
        x as _
    }
    // else  begin x:=x+@'10000000000;
    else {
        let mut x_u: word;
        x_u = (x + 0o10000000000) as _;
        // x:=x+@'10000000000;
        x_u += 0o10000000000;
        // dvi_out((x div @'100000000) + 128);
        dvi_out!(globals, (x_u / 0o100000000) + 128);
        // end;
        x_u
    };
    // x:=x mod @'100000000; dvi_out(x div @'200000);
    x %= 0o100000000;
    dvi_out!(globals, x / 0o200000);
    // x:=x mod @'200000; dvi_out(x div @'400);
    x %= 0o200000;
    dvi_out!(globals, x / 0o400);
    // dvi_out(x mod @'400);
    dvi_out!(globals, x % 0o400);
    // end;
}

use crate::pascal::integer;
use crate::pascal::word;
use crate::section_0004::TeXGlobals;
use crate::section_0598::dvi_out;
