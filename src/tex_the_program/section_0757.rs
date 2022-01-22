//! @ When there is a subscript without a superscript, the top of the subscript
//! should not exceed the baseline plus four-fifths of the x-height.
//
// @<Construct a subscript box |x| when there is no superscript@>=
pub(crate) macro Construct_a_subscript_box_x_when_there_is_no_superscript($globals:expr, $x:expr, $q:expr, $shift_down:expr, $clr:expr) {{
    // begin x:=clean_box(subscr(q),sub_style(cur_style));
    $x = clean_box($globals, subscr!($q), sub_style!($globals.cur_style).into())?;
    // width(x):=width(x)+script_space;
    width!($globals, $x) += script_space!($globals);
    // if shift_down<sub1(cur_size) then shift_down:=sub1(cur_size);
    if $shift_down < sub1!($globals, $globals.cur_size.get()) {
        $shift_down = sub1!($globals, $globals.cur_size.get());
    }
    // clr:=height(x)-(abs(math_x_height(cur_size)*4) div 5);
    $clr = height!($globals, $x)
        - scaled::new_from_inner(
            (math_x_height!($globals, $globals.cur_size.get()).inner() * 4).abs() / 5,
        );
    // if shift_down<clr then shift_down:=clr;
    if $shift_down < $clr {
        $shift_down = $clr;
    }
    // shift_amount(x):=shift_down;
    shift_amount!($globals, $x) = $shift_down;
    // end
    use crate::section_0101::scaled;
    use crate::section_0135::height;
    use crate::section_0135::shift_amount;
    use crate::section_0135::width;
    use crate::section_0247::script_space;
    use crate::section_0681::subscr;
    use crate::section_0700::math_x_height;
    use crate::section_0700::sub1;
    use crate::section_0702::sub_style;
    use crate::section_0720::clean_box;
}}
