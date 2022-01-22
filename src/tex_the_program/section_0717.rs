//! @ The |math_kern| subroutine removes |mu_glue| from a kern node, given
//! the value of the math unit.
//
// @p procedure math_kern(@!p:pointer;@!m:scaled);
pub(crate) fn math_kern(globals: &mut TeXGlobals, p: pointer, m: scaled) {
    // var @!n:integer; {integer part of |m|}
    /// integer part of `m`
    let mut n;
    // @!f:scaled; {fraction part of |m|}
    /// fraction part of `m`
    let mut f;
    // begin if subtype(p)=mu_glue then
    if subtype!(globals, p) == kern_node_subtype::mu_glue as _ {
        // begin n:=x_over_n(m,@'200000); f:=remainder;@/
        n = x_over_n(globals, m, 0o200000).inner();
        f = globals.remainder;
        // if f<0 then
        if f < scaled::zero() {
            // begin decr(n); f:=f+@'200000;
            decr!(n);
            f += scaled::new_from_inner(0o200000);
            // end;
        }
        // width(p):=mu_mult(width(p)); subtype(p):=explicit;
        width!(globals, p) = mu_mult!(globals, width!(globals, p), n, f);
        subtype!(globals, p) = kern_node_subtype::explicit as _;
        // end;
    }
    // end;
}

use crate::section_0004::TeXGlobals;
use crate::section_0016::decr;
use crate::section_0101::scaled;
use crate::section_0106::x_over_n;
use crate::section_0115::pointer;
use crate::section_0133::subtype;
use crate::section_0135::width;
use crate::section_0155::kern_node_subtype;
use crate::section_0716::mu_mult;
