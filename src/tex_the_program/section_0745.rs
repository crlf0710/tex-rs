//! @ The numerator and denominator must be separated by a certain minimum
//! clearance, called |clr| in the following program. The difference between
//! |clr| and the actual clearance is twice |delta|.
//
// @<Adjust \(s)|shift_up| and |shift_down| for the case of no fraction line@>=
pub(crate) macro Adjust_shift_up_and_shift_down_for_the_case_of_no_fraction_line($globals:expr, $x:expr, $z:expr, $shift_up:expr, $shift_down:expr, $delta:expr) {{
    /// dimensions for box calculations
    let clr;
    // begin if cur_style<text_style then clr:=7*default_rule_thickness
    if $globals.cur_style.get() < style_node_subtype::text_style.get() {
        clr = scaled::new_from_inner(7 * default_rule_thickness!($globals).inner());
    }
    // else clr:=3*default_rule_thickness;
    else {
        clr = scaled::new_from_inner(3 * default_rule_thickness!($globals).inner());
    }
    // delta:=half(clr-((shift_up-depth(x))-(height(z)-shift_down)));
    $delta =
        (clr - (($shift_up - depth!($globals, $x)) - (height!($globals, $z) - $shift_down))).half();
    // if delta>0 then
    if $delta > scaled::zero() {
        // begin shift_up:=shift_up+delta;
        $shift_up += $delta;
        // shift_down:=shift_down+delta;
        $shift_down += $delta;
        // end;
    }
    // end
    use crate::section_0100::Half;
    use crate::section_0101::scaled;
    use crate::section_0135::depth;
    use crate::section_0135::height;
    use crate::section_0688::style_node_subtype;
    use crate::section_0701::default_rule_thickness;
}}
