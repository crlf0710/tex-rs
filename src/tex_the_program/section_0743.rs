//! @ The |make_fraction| procedure is a bit different because it sets
//! |new_hlist(q)| directly rather than making a sub-box.
//
// @<Declare math...@>=
// procedure make_fraction(@!q:pointer);
pub(crate) fn make_fraction(globals: &mut TeXGlobals, q: pointer) {
    // var p,@!v,@!x,@!y,@!z:pointer; {temporary registers for box construction}
    // @!delta,@!delta1,@!delta2,@!shift_up,@!shift_down,@!clr:scaled;
    //   {dimensions for box calculations}
    // begin if thickness(q)=default_code then thickness(q):=default_rule_thickness;
    if thickness!(globals, q) == default_code {
        thickness!(globals, q) = default_rule_thickness!(globals);
    }
    // @<Create equal-width boxes |x| and |z| for the numerator and denominator,
    //   and compute the default amounts |shift_up| and |shift_down| by which they
    //   are displaced from the baseline@>;
    todo!("Create equal-width boxes");
    // if thickness(q)=0 then @<Adjust \(s)|shift_up| and |shift_down| for the case
    //   of no fraction line@>
    if thickness!(globals, q) == scaled::zero() {
        todo!("Adjust shift_up and shift_down");
    }
    // else @<Adjust \(s)|shift_up| and |shift_down| for the case of a fraction line@>;
    else {
        todo!("Adjust shift_up and shift_down for fraction line");
    }
    // @<Construct a vlist box for the fraction, according to |shift_up| and
    //   |shift_down|@>;
    // @<Put the \(f)fraction into a box with its delimiters, and make |new_hlist(q)|
    //   point to it@>;
    // end;
}

use crate::section_0004::TeXGlobals;
use crate::section_0101::scaled;
use crate::section_0115::pointer;
use crate::section_0683::default_code;
use crate::section_0683::thickness;
use crate::section_0701::default_rule_thickness;
