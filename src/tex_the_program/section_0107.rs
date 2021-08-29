//! @ Then comes the multiplication of a scaled number by a fraction |n/d|,
//! where |n| and |d| are nonnegative integers |<=@t$2^{16}$@>| and |d| is
//! positive. It would be too dangerous to multiply by~|n| and then divide
//! by~|d|, in separate operations, since overflow might well occur; and it
//! would be too inaccurate to divide by |d| and then multiply by |n|. Hence
//! this subroutine simulates 1.5-precision arithmetic.
//
// @p function xn_over_d(@!x:scaled; @!n,@!d:integer):scaled;
pub(crate) fn xn_over_d(globals: &mut TeXGlobals, mut x: scaled, n: integer, d: integer) -> scaled {
    // var positive:boolean; {was |x>=0|?}
    /// was `x>=0`?
    let positive: boolean;
    // @!t,@!u,@!v:nonnegative_integer; {intermediate quantities}
    /// intermediate quantities
    let (t, mut u, v): (
        nonnegative_integer,
        nonnegative_integer,
        nonnegative_integer,
    );
    // begin if x>=0 then positive:=true
    if x >= scaled::zero() {
        positive = true;
    }
    // else  begin negate(x); positive:=false;
    else {
        negate!(x);
        positive = false;
        // end;
    };
    // t:=(x mod @'100000)*n;
    t = ((x.inner() % 0o100000) * n).into();
    // u:=(x div @'100000)*n+(t div @'100000);
    u = ((x.inner() / 0o100000) * n + (t.get() / 0o100000)).into();
    // v:=(u mod d)*@'100000 + (t mod @'100000);
    v = ((u.get() % d) * 0o100000 + (t.get() % 0o100000)).into();
    // if u div d>=@'100000 then arith_error:=true
    if u.get() / d >= 0o100000 {
        globals.arith_error = true
    }
    // else u:=@'100000*(u div d) + (v div d);
    else {
        u = (0o100000 * (u.get() / d) + (v.get() / d)).into();
    }
    let xn_over_d;
    // if positive then
    if positive {
        // begin xn_over_d:=u; remainder:=v mod d;
        xn_over_d = u.get() as integer;
        globals.remainder = scaled::new_from_inner(v.get() % d);
        // end
    }
    // else  begin xn_over_d:=-u; remainder:=-(v mod d);
    else {
        xn_over_d = -(u.get() as integer);
        globals.remainder = scaled::new_from_inner(-(v.get() % d));
        // end;
    }
    // end;
    scaled::new_from_inner(xn_over_d)
}

use crate::pascal::boolean;
use crate::pascal::integer;
use crate::section_0004::TeXGlobals;
use crate::section_0101::nonnegative_integer;
use crate::section_0101::scaled;
