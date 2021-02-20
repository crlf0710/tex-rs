//! @ The next subroutine is used to compute the ``badness'' of glue, when a
//! total~|t| is supposed to be made from amounts that sum to~|s|.  According
//! to {\sl The \TeX book}, the badness of this situation is $100(t/s)^3$;
//! however, badness is simply a heuristic, so we need not squeeze out the
//! last drop of accuracy when computing it. All we really want is an
//! approximation that has similar properties.
//! @:TeXbook}{\sl The \TeX book@>
//!
//! The actual method used to compute the badness is easier to read from the
//! program than to describe in words. It produces an integer value that is a
//! reasonably close approximation to $100(t/s)^3$, and all implementations
//! of \TeX\ should use precisely this method. Any badness of $2^{13}$ or more is
//! treated as infinitely bad, and represented by 10000.
//!
//! It is not difficult to prove that $$\hbox{|badness(t+1,s)>=badness(t,s)
//! >=badness(t,s+1)|}.$$ The badness function defined here is capable of
//! computing at most 1095 distinct values, but that is plenty.
//
// @d inf_bad = 10000 {infinitely bad value}
/// infinitely bad value
pub(crate) const inf_bad: halfword = 10000;

// @p function badness(@!t,@!s:scaled):halfword; {compute badness, given |t>=0|}
/// compute badness, given `t>=0`
#[allow(unused_variables)]
#[cfg_attr(feature = "trace", tracing::instrument(level = "trace", skip(globals)))]
pub(crate) fn badness(globals: &mut TeXGlobals, t: scaled, s:scaled) -> halfword {
    // var r:integer; {approximation to $\alpha t/s$, where $\alpha^3\approx
    //   100\cdot2^{18}$}
    // begin if t=0 then badness:=0
    if t == scaled::zero() {
        return 0;
    }
    // else if s<=0 then badness:=inf_bad
    else if s <= scaled::zero() {
        return inf_bad;
    }
    // else  begin if t<=7230584 then  r:=(t*297) div s {$297^3=99.94\times2^{18}$}
    else {
        /// approximation to `αt/s`, where `α^3≈100·2^18`
        let r: integer;
        if t.inner() <= 7230584 {
            /// `297^3=99.94*2^{18}`
            const _: () = ();
            r = t.inner() * 297 / s.inner();
        }
        // else if s>=1663497 then r:=t div (s div 297)
        else if s.inner() >= 1663497 {
            r = t.inner() / (s.inner() / 297);
        }
        // else r:=t;
        else {
            r = t.inner();
        }
        // if r>1290 then badness:=inf_bad {$1290^3<2^{31}<1291^3$}
        if r > 1290 {
            /// `1290^3<2^{31}<1291^3`
            const _: () = ();
            return inf_bad;
        }
        // else badness:=(r*r*r+@'400000) div @'1000000;
        else {
            return ((r * r * r + 0o400000) / 0o1000000) as _;
            /// that was `r^3/2^{18}`, rounded to the nearest integer
            const _: () = ();
        }
        // end; {that was $r^3/2^{18}$, rounded to the nearest integer}
    }
    // end;
}

use crate::pascal::integer;
use crate::section_0004::TeXGlobals;
use crate::section_0101::scaled;
use crate::section_0113::halfword;
