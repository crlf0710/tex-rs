//! @ The |make_fraction| procedure is a bit different because it sets
//! |new_hlist(q)| directly rather than making a sub-box.
//
// @<Declare math...@>=
// procedure make_fraction(@!q:pointer);
#[allow(unused_variables, unused_assignments)]
pub(crate) fn make_fraction(globals: &mut TeXGlobals, q: pointer) -> TeXResult<()> {
    // var p,@!v,@!x,@!y,@!z:pointer; {temporary registers for box construction}
    /// temporary registers for box construction
    let (v, mut x, mut z);
    // @!delta,@!delta1,@!delta2,@!shift_up,@!shift_down,@!clr:scaled;
    //   {dimensions for box calculations}
    /// dimensions for box calculations
    let (delta, mut shift_up, mut shift_down);
    // begin if thickness(q)=default_code then thickness(q):=default_rule_thickness;
    if thickness!(globals, q) == default_code {
        thickness!(globals, q) = default_rule_thickness!(globals);
    }
    // @<Create equal-width boxes |x| and |z| for the numerator and denominator,
    //   and compute the default amounts |shift_up| and |shift_down| by which they
    //   are displaced from the baseline@>;
    crate::section_0744::Create_equal_width_boxes_x_and_z_for_the_numerator_and_denominator__and_compute_the_default_amounts_shift_up_and_shift_down_by_which_they_are_displaced_from_the_baseline!(
        globals, q, x, z, shift_up, shift_down
    );
    // if thickness(q)=0 then @<Adjust \(s)|shift_up| and |shift_down| for the case
    //   of no fraction line@>
    if thickness!(globals, q) == scaled::zero() {
        todo!("Adjust shift_up and shift_down");
    }
    // else @<Adjust \(s)|shift_up| and |shift_down| for the case of a fraction line@>;
    else {
        crate::section_0746::Adjust_shift_up_and_shift_down_for_the_case_of_a_fraction_line!(
            globals, q, x, z, shift_up, shift_down, delta
        );
    }
    // @<Construct a vlist box for the fraction, according to |shift_up| and
    //   |shift_down|@>;
    crate::section_0747::Construct_a_vlist_box_for_the_fraction__according_to_shift_up_and_shift_down!(
        globals, q, x, z, v, shift_up, shift_down, delta
    );
    // @<Put the \(f)fraction into a box with its delimiters, and make |new_hlist(q)|
    //   point to it@>;
    crate::section_0748::Put_the_fraction_into_a_box_with_its_delimiters__and_make_new_hlist_q_point_to_it!(
        globals, q, v
    );
    // end;
    crate::ok_nojump!()
}

use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0101::scaled;
use crate::section_0115::pointer;
use crate::section_0683::default_code;
use crate::section_0683::thickness;
use crate::section_0701::default_rule_thickness;
