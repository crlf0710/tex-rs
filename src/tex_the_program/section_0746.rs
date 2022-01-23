//! @ In the case of a fraction line, the minimum clearance depends on the actual
//! thickness of the line.
//
// @<Adjust \(s)|shift_up| and |shift_down| for the case of a fraction line@>=
pub(crate) macro Adjust_shift_up_and_shift_down_for_the_case_of_a_fraction_line($globals:expr, $q:expr, $x:expr, $z:expr, $shift_up:expr, $shift_down:expr, $delta:expr) {{
    /// dimensions for box calculations
    let (delta1, delta2, clr);
    // begin if cur_style<text_style then clr:=3*thickness(q)
    if $globals.cur_style.get() < style_node_subtype::text_style.get() {
        clr = scaled::new_from_inner(3 * thickness!($globals, $q).inner());
    }
    // else clr:=thickness(q);
    else {
        clr = thickness!($globals, $q);
    }
    // delta:=half(thickness(q));
    $delta = thickness!($globals, $q);
    // delta1:=clr-((shift_up-depth(x))-(axis_height(cur_size)+delta));
    delta1 = clr
        - (($shift_up - depth!($globals, $x))
            - (axis_height!($globals, $globals.cur_size.get()) + $delta));
    // delta2:=clr-((axis_height(cur_size)-delta)-(height(z)-shift_down));
    delta2 = clr
        - ((axis_height!($globals, $globals.cur_size.get()) - $delta)
            - (height!($globals, $z) - $shift_down));
    // if delta1>0 then shift_up:=shift_up+delta1;
    if delta1 > scaled::zero() {
        $shift_up += delta1;
    }
    // if delta2>0 then shift_down:=shift_down+delta2;
    if delta2 > scaled::zero() {
        $shift_down += delta2;
    }
    // end
    use crate::section_0101::scaled;
    use crate::section_0135::depth;
    use crate::section_0135::height;
    use crate::section_0683::thickness;
    use crate::section_0688::style_node_subtype;
    use crate::section_0700::axis_height;
}}
