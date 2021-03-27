//! @ We also need to divide scaled dimensions by integers.
//
// @p function x_over_n(@!x:scaled;@!n:integer):scaled;
pub(crate) fn x_over_n(globals: &mut TeXGlobals, mut x: scaled, mut n: integer) -> scaled {
    // var negative:boolean; {should |remainder| be negated?}
    /// should `remainder` be negated?
    let mut negative: boolean;

    let x_over_n: scaled;
    // begin negative:=false;
    negative = false;
    // if n=0 then
    if n == 0 {
        // begin arith_error:=true; x_over_n:=0; remainder:=x;
        globals.arith_error = true;
        x_over_n = scaled::zero();
        globals.remainder = x;
        // end
    }
    // else  begin if n<0 then
    else {
        if n < 0 {
            // begin negate(x); negate(n); negative:=true;
            negate!(x);
            negate!(n);
            negative = true;
            // end;
        }
        // if x>=0 then
        if x >= scaled::zero() {
            // begin x_over_n:=x div n; remainder:=x mod n;
            x_over_n = scaled::new_from_inner(x.inner() / n);
            globals.remainder = scaled::new_from_inner(x.inner() % n);
            // end
        }
        // else  begin x_over_n:=-((-x) div n); remainder:=-((-x) mod n);
        else {
            x_over_n = scaled::new_from_inner(-((-x.inner()) / n));
            globals.remainder = scaled::new_from_inner(-((-x.inner()) % n));
            // end;
        }
        // end;
    }
    // if negative then negate(remainder);
    if negative {
        negate!(globals.remainder);
    }
    // end;
    return x_over_n;
}

use crate::pascal::boolean;
use crate::pascal::integer;
use crate::section_0004::TeXGlobals;
use crate::section_0101::scaled;
