//! @ The first arithmetical subroutine we need computes $nx+y$, where |x|
//! and~|y| are |scaled| and |n| is an integer. We will also use it to
//! multiply integers.
//
// @d nx_plus_y(#)==mult_and_add(#,@'7777777777)
macro_rules! nx_plus_y {
    ($globals:expr, $n:expr, $x:expr, $y:expr) => {
        crate::section_0105::mult_and_add(
            $globals,
            $n,
            $x,
            $y,
            crate::section_0101::scaled::new_from_inner(0o7777777777),
        )
    };
}
// @d mult_integers(#)==mult_and_add(#,0,@'17777777777)
//
// @p function mult_and_add(@!n:integer;@!x,@!y,@!max_answer:scaled):scaled;
pub(crate) fn mult_and_add(
    globals: &mut TeXGlobals,
    mut n: integer,
    mut x: scaled,
    y: scaled,
    max_answer: scaled,
) -> scaled {
    // begin if n<0 then
    if n < 0 {
        // begin negate(x); negate(n);
        negate!(x);
        negate!(n);
        // end;
    }
    let mult_and_add;
    // if n=0 then mult_and_add:=y
    if n == 0 {
        mult_and_add = y;
    }
    // else if ((x<=(max_answer-y) div n)and(-x<=(max_answer+y) div n)) then
    else if (x.inner() <= (max_answer.inner() - y.inner()) / n)
        && (-x.inner() <= (max_answer.inner() + y.inner()) / n)
    {
        // mult_and_add:=n*x+y
        mult_and_add = scaled::new_from_inner(n * x.inner() + y.inner());
    }
    // else  begin arith_error:=true; mult_and_add:=0;
    else {
        globals.arith_error = true;
        mult_and_add = scaled::zero();
        // end;
    }
    // end;
    mult_and_add
}

use crate::pascal::integer;
use crate::section_0004::TeXGlobals;
use crate::section_0101::scaled;
